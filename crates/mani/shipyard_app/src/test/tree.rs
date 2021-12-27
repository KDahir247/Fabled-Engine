//! # Tree
//!
//! Tree provides a very simple way to manipulate the ordering of elements in a CRDT fashion enabling
//! a minimal amount of effort to reorder entities and resolve multiple concurrent reorderings.
//!
//! This is intended to be used alongside an additional decorating system which can index the child positions
//! in the tree whenever a movement occurs and to keep everything in check.
//!
//! So, you would have a tree_reordering system (for managing reordering commands operating on the tree components) and
//! a tree_indexing system for updating the tree_index components.
//!
//! The Advantages to this approach are to
//!  - simplify the source of truth for more reliable ser & de
//!  - Reduce the size of the seriallized form
//!  - Less blocking systems (if something only cares that the ChildOf / Ordering has changed and the system does not
//!    look at the indexed outputs, then it can run concurrently with the tree_indexing system)
use crate::*;

mod indexing;
mod node;
mod reordering;

pub use indexing::{ParentIndex, SiblingIndex};
pub use node::*;
pub use reordering::{MoveCmd, MoveToPlace};

#[derive(Component)]
pub struct MoveCommands(Vec<MoveCmd>);

/// Registers
#[derive(Default)]
pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut AppBuilder) {
        // needs direct update pack since TreePlugin clears updates on its own.
        app.update_pack::<ChildOf>("update in response to ChildOf changes")
            .add_system(indexing::tree_indexing);
    }
}

/// A spattering of tests to check things
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_world_with_index_system() -> World {
        // Create a new world
        let world = World::new();

        // Add the indexing workload
        let indexing = WorkloadBuilder::new("indexing");

        indexing
            .with_system(indexing::tree_indexing)
            .with_system(|mut vm_child_of: ViewMut<ChildOf>| {
                vm_child_of.clear_all_inserted_and_modified();
                vm_child_of.take_deleted();
            })
            .add_to_world(&world)
            .unwrap();

        world
    }

    #[test]
    fn insert_single_entity_with_no_children() {
        let world = setup_world_with_index_system();
        // Add a parent and child entity
        let a = world
            .run(|mut entities: EntitiesViewMut| entities.add_entity((), ()))
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // There should be no indexes at all
        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    assert_eq!(v_parent_index.contains(a), false);
                    assert_eq!(v_sibling_index.contains(a), false);
                },
            )
            .unwrap();
    }

    #[test]
    fn insert_single_child() {
        let world = setup_world_with_index_system();
        // Add a parent and child entity
        let (a, a1) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                    (a, a1)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Run an anonymous system to verify the state of the indexes
        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // The root node should not have a sibling index
                    v_sibling_index
                        .get(a)
                        .expect_err("should not have sibling data");

                    // The root node should have a parent index
                    let c: &ParentIndex = v_parent_index.get(a).expect("has children");

                    // The root node should have exactly one child
                    assert_eq!(parent_children_ids(c), vec![a1], "should be in order");

                    // The child should not have a parent index
                    assert_eq!(v_parent_index.contains(a1), false);

                    // The child should have a sibling index, with no siblings and the parent being the root node
                    let a1_sib: &SiblingIndex =
                        v_sibling_index.get(a1).expect("should have sibling data");
                    assert_eq!(a1_sib.prev_sibling, None, "only child");
                    assert_eq!(a1_sib.next_sibling, None, "only child");
                    assert_eq!(a1_sib.parent_node, a)
                },
            )
            .unwrap();
    }

    #[test]
    fn insert_many_siblings() {
        let world = setup_world_with_index_system();
        // Add a parent entity with many children
        let (a, a1, a2, a3) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                    let a2 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(2)));
                    let a3 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(3)));

                    (a, a1, a2, a3)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Run an anonymous system to verify the state of the indexes
        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // The root node should not have a sibling index
                    v_sibling_index
                        .get(a)
                        .expect_err("should not have sibling data");

                    // The root node should have a parent index
                    let c: &ParentIndex = v_parent_index.get(a).expect("has children");

                    // The root node should have many children
                    assert_eq!(
                        parent_children_ids(c),
                        vec![a1, a2, a3],
                        "should be in order"
                    );

                    // The children should not have a parent indexes
                    assert_eq!(v_parent_index.contains(a1), false);
                    assert_eq!(v_parent_index.contains(a2), false);
                    assert_eq!(v_parent_index.contains(a3), false);

                    // The first sibling should not have a prev sibling but have a next sibling
                    let a1_sib = v_sibling_index.get(a1).expect("should have sibling data");
                    assert_eq!(a1_sib.prev_sibling, None, "only child");
                    assert_eq!(a1_sib.next_sibling.unwrap().1, a2, "has sibling");

                    // The middle sibling should have both prev and next siblings
                    let a2_sib = v_sibling_index.get(a2).expect("should have sibling data");
                    assert_eq!(a2_sib.prev_sibling.unwrap().1, a1, "has sibling");
                    assert_eq!(a2_sib.next_sibling.unwrap().1, a3, "has sibling");

                    // The last sibling should have a prev sibling but not have a next sibling
                    let a3_sib = v_sibling_index.get(a3).expect("should have sibling data");
                    assert_eq!(a3_sib.prev_sibling.unwrap().1, a2, "has sibling");
                    assert_eq!(a3_sib.next_sibling, None, "only child");
                },
            )
            .unwrap();
    }

    #[test]
    fn insert_deep_children() {
        let world = setup_world_with_index_system();
        let (a, a1, a2, a3) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                    let a2 = entities.add_entity(&mut vm_child_of, ChildOf(a1, Ordered::hinted(2)));
                    let a3 = entities.add_entity(&mut vm_child_of, ChildOf(a2, Ordered::hinted(3)));

                    (a, a1, a2, a3)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Run an anonymous system to verify the state of the indexes
        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // Each entity should have a parent index containing one child
                    let a_pi: &ParentIndex = v_parent_index.get(a).expect("has children");
                    assert_eq!(parent_children_ids(a_pi), vec![a1]);

                    let a1_pi: &ParentIndex = v_parent_index.get(a1).expect("has children");
                    assert_eq!(parent_children_ids(a1_pi), vec![a2]);

                    let a2_pi: &ParentIndex = v_parent_index.get(a2).expect("has children");
                    assert_eq!(parent_children_ids(a2_pi), vec![a3]);

                    // The leaf entity should not have a parent index
                    assert_eq!(v_parent_index.contains(a3), false);

                    // Each non-root entity should have a sibling index that is empty
                    let a1_sib = v_sibling_index.get(a1).expect("should have sibling data");
                    assert_eq!(a1_sib.prev_sibling, None, "only child");
                    assert_eq!(a1_sib.next_sibling, None, "only child");

                    let a2_sib = v_sibling_index.get(a2).expect("should have sibling data");
                    assert_eq!(a2_sib.prev_sibling, None, "only child");
                    assert_eq!(a2_sib.next_sibling, None, "only child");

                    let a3_sib = v_sibling_index.get(a3).expect("should have sibling data");
                    assert_eq!(a3_sib.prev_sibling, None, "only child");
                    assert_eq!(a3_sib.next_sibling, None, "only child");
                },
            )
            .unwrap();
    }

    #[test]
    // TODO: Complete this test
    fn update_entity_as_child_of_self() {
        let world = setup_world_with_index_system();
        world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    entities.add_component(a, &mut vm_child_of, ChildOf(a, Ordered::hinted(1)));

                    a
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();
    }

    #[test]
    fn delete_only_child() {
        let world = setup_world_with_index_system();
        // Add a parent and child entity
        let (a, a1) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));

                    (a, a1)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Delete the child entity
        world
            .run(|mut vm_child_of: ViewMut<ChildOf>| {
                // remove should not be used as it is untracked for whatever reason...
                vm_child_of.delete(a1);
            })
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // There should be no sibling index for the previous parent
                    assert_eq!(v_sibling_index.contains(a), false);

                    // TODO: Should there be a parent index any more with no children?
                    // There should be a parent index with no children
                    let c: &ParentIndex = v_parent_index.get(a).expect("has a parent index");
                    assert_eq!(parent_children_ids(c), vec![]);
                },
            )
            .unwrap();
    }

    #[test]
    fn delete_first_and_last_siblings() {
        let world = setup_world_with_index_system();
        let (a, a1, a2, a3) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                    let a2 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(2)));
                    let a3 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(3)));

                    (a, a1, a2, a3)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Delete the first and last siblings
        world
            .run(|mut vm_child_of: ViewMut<ChildOf>| {
                vm_child_of.delete(a1);
                vm_child_of.delete(a3);
            })
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // The root should only have one child now
                    let c: &ParentIndex = v_parent_index.get(a).expect("has a parent index");
                    assert_eq!(parent_children_ids(c), vec![a2]);

                    // The previously first and last siblings should no longer have an index
                    assert_eq!(v_sibling_index.contains(a1), false);
                    assert_eq!(v_sibling_index.contains(a3), false);

                    // The previously middle node should now not have any siblings
                    let a2_sib = v_sibling_index.get(a2).expect("should have sibling data");
                    assert_eq!(a2_sib.prev_sibling, None, "only child");
                    assert_eq!(a2_sib.next_sibling, None, "only child");
                },
            )
            .unwrap();
    }

    #[test]
    fn delete_middle_sibling() {
        let world = setup_world_with_index_system();
        let (a, a1, a2, a3) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                    let a2 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(2)));
                    let a3 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(3)));

                    (a, a1, a2, a3)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Delete the first and last siblings
        world
            .run(|mut vm_child_of: ViewMut<ChildOf>| {
                vm_child_of.delete(a2);
            })
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // The root should have 2 children now
                    let c: &ParentIndex = v_parent_index.get(a).expect("has a parent index");
                    assert_eq!(parent_children_ids(c), vec![a1, a3]);

                    // The previously middle sibling should no longer have an index
                    assert_eq!(v_sibling_index.contains(a2), false);

                    // The previously first and last siblings should now be connected
                    let a1_sib = v_sibling_index.get(a1).expect("should have sibling data");
                    assert_eq!(a1_sib.next_sibling.unwrap().1, a3);

                    let a3_sib = v_sibling_index.get(a3).expect("should have sibling data");
                    assert_eq!(a3_sib.prev_sibling.unwrap().1, a1);
                },
            )
            .unwrap();
    }

    #[test]
    fn delete_middle_ancestor() {
        let world = setup_world_with_index_system();
        let (a, a1, a2) = world
            .run(
                |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                    let a = entities.add_entity((), ());
                    let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                    let a2 = entities.add_entity(&mut vm_child_of, ChildOf(a1, Ordered::hinted(2)));

                    (a, a1, a2)
                },
            )
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // Delete the first and last siblings
        world
            .run(|mut vm_child_of: ViewMut<ChildOf>| {
                vm_child_of.delete(a1);
            })
            .unwrap();

        // Run the indexing workload
        world.run_default().unwrap();

        // TODO: determine the correct behaviour for reparenting (do all descendents get deleted, do children get reparented?)
        world
            .run(
                |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                    // The root now has no children
                    let c: &ParentIndex = v_parent_index.get(a).expect("has a parent index");
                    assert_eq!(parent_children_ids(c), vec![]);

                    // The previous middle ancestor should no longer have a sibling index
                    assert_eq!(v_parent_index.contains(a1), true);
                    assert_eq!(v_sibling_index.contains(a1), false);

                    // The previous grandchild still references its parent
                    let a2_sib = v_sibling_index.get(a2).expect("should have sibling data");
                    assert_eq!(a2_sib.parent_node, a1);

                    // Potential test code for descendent deletion
                    // The root should have no children
                    // let c: &ParentIndex = v_parent_index.get(a).expect("has a parent index");
                    // assert_eq!(parent_children_ids(c), vec![]);

                    // The previous middle ancestor and its descendents should no longer have an index
                    // assert_eq!(v_parent_index.contains(a1), false);
                    // assert_eq!(v_sibling_index.contains(a1), false);

                    // assert_eq!(v_parent_index.contains(a2), false);
                    // assert_eq!(v_sibling_index.contains(a2), false);

                    // Potential test code for reparenting:
                    // The root should have the grandchild as a child now
                    // let c: &ParentIndex = v_parent_index.get(a).expect("has a parent index");
                    // assert_eq!(parent_children_ids(c), vec![a2]);

                    // The previous middle ancestor should no longer have an index
                    // assert_eq!(v_parent_index.contains(a1), false);
                    // assert_eq!(v_sibling_index.contains(a1), false);

                    // // The previous grandchild should now have the root as a parent
                    // let a2_sib = v_sibling_index.get(a2).expect("should have sibling data");
                    // assert_eq!(a2_sib.parent_node, a);
                },
            )
            .unwrap();
    }

    #[test]
    fn test_indexing() {
        let app = App::new();

        WorkloadBuilder::new("default")
            .with_system(indexing::tree_indexing)
            .with_system(|mut vm_child_of: ViewMut<ChildOf>| {
                vm_child_of.clear_all_inserted_and_modified();
            })
            .add_to_world(&app.world)
            .unwrap();

        test_with_indexing_with_world(app);
    }

    #[test]
    fn test_indexing_with_plugin() {
        let app = App::new();
        let mut builder = AppBuilder::new(&app);
        builder.add_plugin(TreePlugin::default());
        builder.finish();
        test_with_indexing_with_world(app);
    }

    fn test_with_indexing_with_world(app: App) {
        let (a, a1, a2, a3, a6) = app.run(
            |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                let a = entities.add_entity((), ());
                let a6 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(6)));
                let a3 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(3)));
                let a1 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(1)));
                let a2 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(2)));
                (a, a1, a2, a3, a6)
            },
        );

        app.update();

        app.run(
            |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                v_sibling_index
                    .get(a)
                    .expect_err("should not have sibling data");

                let c: &ParentIndex = v_parent_index.get(a).expect("has children");

                assert_eq!(
                    parent_children_ids(c),
                    vec![a1, a2, a3, a6],
                    "should be in order"
                );
            },
        );

        let (a1b, a1b1, a0, a4, a7) = app.run(
            |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                let a7 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(7)));
                let a0 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(0)));
                let a4 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(4)));
                let a1b = entities.add_entity(&mut vm_child_of, ChildOf(a1, Ordered::hinted(4)));
                let a1b1 = entities.add_entity(&mut vm_child_of, ChildOf(a1b, Ordered::hinted(1)));
                (a1b, a1b1, a0, a4, a7)
            },
        );

        app.update();

        app.run(
            |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                v_sibling_index
                    .get(a)
                    .expect_err("should not have sibling data");

                let a1b_sib: &SiblingIndex =
                    v_sibling_index.get(a1b).expect("should have sibling data");

                assert_eq!(a1b_sib.next_sibling, None, "only child");
                assert_eq!(a1b_sib.prev_sibling, None, "only child");

                assert_eq!(
                    parent_children_ids(v_parent_index.get(a1b).expect("has children")),
                    vec![a1b1],
                    "should have the one child"
                );

                assert_eq!(
                    parent_children_ids(v_parent_index.get(a).expect("has children")),
                    vec![a0, a1, a2, a3, a4, a6, a7],
                    "should be in order"
                );
            },
        );

        // test removing and deleting ChildOf components

        app.run(|mut vm_child_of: ViewMut<ChildOf>| {
            // remove should not be used as it is untracked for whatever reason...
            vm_child_of.delete(a7);
            vm_child_of.delete(a4);
            vm_child_of.delete(a0);
            vm_child_of.delete(a1b);
        });

        app.update();

        app.run(
            |v_parent_index: View<ParentIndex>, v_sibling_index: View<SiblingIndex>| {
                v_sibling_index
                    .get(a1b)
                    .expect_err("should not have sibling data");

                assert_eq!(
                    parent_children_ids(v_parent_index.get(a1b).expect("has children")),
                    vec![a1b1],
                    "should have the one child"
                );

                assert_eq!(
                    parent_children_ids(v_parent_index.get(a).expect("has children")),
                    vec![a1, a2, a3, a6],
                    "should be in order without deleted entities"
                );
            },
        );

        // test inserting & overwriting after deleting

        // a2 updated to ord 7, a1 child of a8
        let (a8,) = app.run(
            |mut entities: EntitiesViewMut, mut vm_child_of: ViewMut<ChildOf>| {
                (&mut vm_child_of).get(a2).unwrap().1 = Ordered::hinted(7);
                let a8 = entities.add_entity(&mut vm_child_of, ChildOf(a, Ordered::hinted(8)));
                (&mut vm_child_of).get(a1).unwrap().0 = a8;

                (a8,)
            },
        );

        app.update();

        app.run(|v_parent_index: View<ParentIndex>| {
            assert_eq!(
                parent_children_ids(v_parent_index.get(a).expect("has children")),
                vec![a3, a6, a2, a8],
                "should be in order with updated entities"
            );
            assert_eq!(
                parent_children_ids(v_parent_index.get(a8).expect("has children")),
                vec![a1],
                "should have modified parent"
            );
        });
    }

    fn parent_children_ids(pi: &ParentIndex) -> Vec<EntityId> {
        pi.children.iter().map(|c| c.1).collect()
    }
}
