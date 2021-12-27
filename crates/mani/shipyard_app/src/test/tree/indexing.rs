use super::*;

// Ordered first in tuple so it takes ordering precedence
type SiblingID = (Ordered, EntityId);

/// Managed by the tree_indexing system to provide more concise info for walking
/// the tree
#[derive(Debug, Component)]
pub struct SiblingIndex {
    pub parent_node: EntityId,
    pub ordered_node: SiblingID,
    pub prev_sibling: Option<SiblingID>,
    pub next_sibling: Option<SiblingID>,
}

/// Managed by the tree_indexing system to provide more concise info for walking
/// the tree
#[derive(Debug, Component)]
pub struct ParentIndex {
    pub children: Vec<SiblingID>,
}

/// Indexes tree [ChildOf] and [Ordering] components into more helpful between
/// nodes
pub fn tree_indexing(
    (v_entities, mut vm_child_of, mut vm_sibling_index, mut vm_parent_index): (
        EntitiesView,
        ViewMut<ChildOf>,
        ViewMut<SiblingIndex>,
        ViewMut<ParentIndex>,
    ),
) {
    println!("hi");
    // iff ChildOf was completely deleted (does not include "removed")
    vm_child_of
        .take_deleted()
        .into_iter()
        .map(|(id, _)| id)
        .for_each(|deleted_id: EntityId| {
            unlink_child(&mut vm_sibling_index, &mut vm_parent_index, deleted_id);
        });

    // iff ChildOf is completely new component
    vm_child_of.inserted().iter().with_id().for_each(
        |(inserted_id, ChildOf(parent_id, child_order))| {
            insert_child_of(
                &v_entities,
                &vm_child_of,
                &mut vm_sibling_index,
                &mut vm_parent_index,
                inserted_id,
                &child_order,
                *parent_id,
            );
        },
    );

    // iff ChildOf was modified
    vm_child_of.modified().iter().with_id().for_each(
        |(modified_id, ChildOf(parent_id, child_order))| {
            // remove from parent
            unlink_child(&mut vm_sibling_index, &mut vm_parent_index, modified_id);

            // reinsert child
            insert_child_of(
                &v_entities,
                &vm_child_of,
                &mut vm_sibling_index,
                &mut vm_parent_index,
                modified_id,
                &child_order,
                *parent_id,
            );
        },
    );

    vm_child_of.clear_all_inserted_and_modified();
}

fn insert_child_of(
    v_entities: &EntitiesView,
    all_child_of_iter: &ViewMut<ChildOf>, /* needed for creating parent node indexes, since
                                           * parents do not need a ChildOf component */
    vm_sibling_index: &mut ViewMut<SiblingIndex>,
    vm_parent_index: &mut ViewMut<ParentIndex>,
    child_id: EntityId,
    child_order: &Ordered, // used to position between siblings
    parent_id: EntityId,   // insert to this parent
) {
    // parent: insert into list at correct location,
    // find next index and previous index and update their sibling references
    // respectively
    let parent_index = {
        if let Ok(parent_index) = vm_parent_index.get(parent_id) {
            parent_index
        } else {
            let mut children = all_child_of_iter
                .iter()
                .with_id()
                .filter(|(_, ChildOf(ref child_parent_id, _))| child_parent_id == &parent_id)
                .map(|(id, ChildOf(_, ref ordered))| -> SiblingID { (*ordered, id) })
                .collect::<Vec<SiblingID>>();

            children.sort();

            // we need to create their SiblingIndex components
            for (idx, child) in children.iter().enumerate() {
                // dbg!(child);
                v_entities.add_component(
                    child.1,
                    &mut *vm_sibling_index,
                    SiblingIndex {
                        next_sibling: if idx < children.len() - 1 {
                            Some(children[idx + 1])
                        } else {
                            None
                        },
                        prev_sibling: if idx > 0 {
                            Some(children[idx - 1])
                        } else {
                            None
                        },
                        ordered_node: *child,
                        parent_node: parent_id,
                    },
                );
            }

            // Good debugging spot if needed
            // for sibling in vm_sibling_index.iter() {
            //     dbg!(sibling);
            // }

            // parent has no parent or siblings
            v_entities.add_component(parent_id, &mut *vm_parent_index, ParentIndex { children });

            vm_parent_index
                .get(parent_id)
                .expect("parent should have a parent index now")
        }
    };

    let siblings = &mut parent_index.children;

    let to_insert: SiblingID = (*child_order, child_id);
    if siblings.binary_search(&to_insert).is_err() {
        // didn't find the sibling_id (ord + id) combo in siblings,
        // this could mean that either the Ordered value changed, or
        // this could mean that the entity is not present in the sibling list
        // at all.

        // remove our id, just in case it was just an "Ordered" change
        siblings.retain(|(_, id)| id != &child_id);

        // "insert_at" points to the index of the element after
        // "insert_at - 1" points to the index of the previous element
        let insert_at = {
            siblings
                .binary_search(&to_insert)
                .expect_err("existing child")
        };

        let (prev_node_opt, next_node_opt) = {
            (
                if insert_at > 0 {
                    // we have an element before to update (which becomes our previous node)
                    Some((&siblings)[insert_at - 1])
                } else {
                    None
                },
                if insert_at < siblings.len() {
                    // we have an element after to update (which becomes our next node)
                    Some((&siblings)[insert_at])
                } else {
                    None
                },
            )
        };

        // insert node into children as final modification to siblings
        siblings.insert(insert_at, to_insert);

        // update references
        if let Some(prev_node) = prev_node_opt {
            // prev node should point at inserted node as next
            (vm_sibling_index.get(prev_node.1).unwrap()).next_sibling = Some(to_insert);
        }

        if let Some(next_node) = next_node_opt {
            // next node should point at inserted node as prev
            (vm_sibling_index.get(next_node.1).unwrap()).prev_sibling = Some(to_insert);
        }

        v_entities.add_component(
            child_id,
            vm_sibling_index,
            SiblingIndex {
                ordered_node: to_insert,
                next_sibling: next_node_opt,
                prev_sibling: prev_node_opt,
                parent_node: parent_id,
            },
        );
    }
}

fn unlink_child(
    vm_sibling_index: &mut ViewMut<SiblingIndex>,
    vm_parent_index: &mut ViewMut<ParentIndex>,
    child: EntityId,
) {
    let (parent_id, t_prev_sibling, t_next_sibling) = {
        let child_index = vm_sibling_index.get(child).unwrap();
        (
            child_index.parent_node,
            child_index.prev_sibling,
            child_index.next_sibling,
        )
    };

    // parent: remove T from children
    let parent_index = vm_parent_index.get(parent_id).unwrap();
    parent_index.children.retain(|(_, id)| id != &child);

    if let Some(prev_sibling_id) = t_prev_sibling {
        // prevsibling: set nextsibling to T's nextsibling
        let mut prev_sibling_index = vm_sibling_index.get(prev_sibling_id.1).unwrap();
        prev_sibling_index.next_sibling = t_next_sibling;
    }

    if let Some(next_sibling_id) = t_next_sibling {
        // nextsibling: set prevsibling to T's prevsibling
        let mut next_sibling_index = vm_sibling_index.get(next_sibling_id.1).unwrap();
        next_sibling_index.prev_sibling = t_prev_sibling;
    }

    vm_sibling_index.delete(child);
}
