#![allow(dead_code)]
use super::*;
use tracing::*;

#[derive(Clone, Debug)]
pub struct MoveCmd {
    pub target: EntityId,
    pub place: MoveToPlace,
}

#[derive(Clone, Debug)]
pub enum MoveToPlace {
    Unlink,
    After(EntityId),
    FirstChildOf(EntityId),
    LastChildOf(EntityId),
}

/// TODO: If there are more than one command, how are we re-indexing to heal the ParentIndex and SiblingIndex?
pub fn tree_reordering(
    (mut commands, mut vm_child_of, v_parent_index, v_sibling_index): (
        UniqueViewMut<MoveCommands>,
        ViewMut<ChildOf>,
        View<ParentIndex>,
        View<SiblingIndex>,
    ),
) {
    let commands = commands.0.drain(..).collect::<Vec<_>>();
    if commands.len() > 1 {
        warn!(
            ?commands,
            "Multiple commands without reindexing, this may result in unexpected moves if the MoveCmds build on each other."
        );
    }

    // NOTE: self after self? Creates a cycle? Do we need to check
    for cmd in commands {
        let span = info_span!("applying move command", ?cmd);
        let _entered = span.enter();
        *(&mut vm_child_of).get(cmd.target).unwrap() = match cmd.place {
            MoveToPlace::After(a) => {
                // check that a & b are both of the same parent
                let ChildOf(a_of, a_ord) = (&vm_child_of).get(a).unwrap();

                let sibling = v_sibling_index.get(*a_of).expect("After command should point at indexed sibling. Perhaps a indexing step was missed.");

                let new_ord = match sibling.next_sibling {
                    Some((next_ord, _)) => Ordered::between(&a_ord, &next_ord),
                    None => Ordered::after(&a_ord),
                };

                ChildOf(*a_of, new_ord)
            }
            MoveToPlace::FirstChildOf(parent) => v_parent_index
                .get(parent)
                .ok()
                .and_then(|parent_index: &ParentIndex| parent_index.children.first())
                .map(|first_child| ChildOf(parent, Ordered::before(&first_child.0)))
                // found no first child in index, create new ChildOf
                .unwrap_or_else(|| ChildOf(parent, Ordered::hinted(0))),
            MoveToPlace::LastChildOf(parent) => v_parent_index
                .get(parent)
                .ok()
                .and_then(|parent_index: &ParentIndex| parent_index.children.last())
                .map(|last_child| ChildOf(parent, Ordered::after(&last_child.0)))
                // found no last child in index, create new ChildOf
                .unwrap_or_else(|| ChildOf(parent, Ordered::hinted(0))),
            MoveToPlace::Unlink => ChildOf(EntityId::dead(), Ordered::hinted(0)),
        }
    }
}
