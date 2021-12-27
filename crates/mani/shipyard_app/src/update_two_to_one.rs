use std::collections::HashSet;

use shipyard::*;

/// # Update two to one
///
/// A shipyard view for when you need to assign some component
/// based upon another component.
///
/// This is called "two-to-one", because you cannot optionally assign the
/// "write" component. If there is a "read" component changed, the "write"
/// component must have a value to be assigned.
///
/// Automatically manages checking for distinct values (if the "write" component
/// == its previous value, then no update). And, this view manages removing the
/// "write" component if the "read" component is removed.
pub struct UpdateTwoToOne<
    'a,
    T1: Component,
    T2: Component,
    U: PartialEq + Component,
    UTrack: track::Tracking<U> = <U as Component>::Tracking,
>(View<'a, T1>, View<'a, T2>, ViewMut<'a, U, UTrack>);

pub struct UpdateTwoToOneBorrower<T1, T2, U>(T1, T2, U);

impl<T1, T2, U> IntoBorrow for UpdateTwoToOne<'_, T1, T2, U>
where
    T1: Send + Sync + Component,
    T2: Send + Sync + Component,
    U: PartialEq + Send + Sync + Component,
    <T1::Tracking as track::Tracking<T1>>::DeletionData: Send + Sync,
    <T2::Tracking as track::Tracking<T2>>::DeletionData: Send + Sync,
    <U::Tracking as track::Tracking<U>>::DeletionData: Send + Sync,
{
    type Borrow = UpdateTwoToOneBorrower<T1, T2, U>;
}

impl<'a, T1, T2, U> Borrow<'a> for UpdateTwoToOneBorrower<T1, T2, U>
where
    T1: Send + Sync + Component,
    T2: Send + Sync + Component,
    U: PartialEq + Send + Sync + Component,
    <T1::Tracking as track::Tracking<T1>>::DeletionData: Send + Sync,
    <T2::Tracking as track::Tracking<T2>>::DeletionData: Send + Sync,
    <U::Tracking as track::Tracking<U>>::DeletionData: Send + Sync,
{
    type View = UpdateTwoToOne<'a, T1, T2, U>;

    fn borrow(world: &'a World) -> Result<Self::View, error::GetStorage> {
        Ok(UpdateTwoToOne(
            world.borrow()?,
            world.borrow()?,
            world.borrow()?,
        ))
    }
}

unsafe impl<'a, T1, T2, U> BorrowInfo for UpdateTwoToOne<'a, T1, T2, U>
where
    T1: Sync + Send + Component,
    T2: Sync + Send + Component,
    U: PartialEq + Sync + Send + Component,
{
    fn borrow_info(mut info: &mut Vec<info::TypeInfo>) {
        View::<'a, T1>::borrow_info(&mut info);
        View::<'a, T2>::borrow_info(&mut info);
        ViewMut::<'a, U>::borrow_info(&mut info);
    }
}

// The compiler has trouble abstracting the types with a bound, so I use a macro
// The impl is the same for all tracking
// The only change is the type returned, `&mut U` if no modification tracking, `Mut<U>` otherwise
macro_rules! impl_update_two_to_one {
    ($(($tracking: ident, $out: ty)),+) => {
        $(
            impl<'a, T1, T2, U> UpdateTwoToOne<'a, T1, T2, U, track::$tracking>
            where
                T1: Sync + Send + Component<Tracking = track::All>,
                T2: Sync + Send + Component<Tracking = track::All>,
                U: PartialEq + Sync + Send + Component<Tracking = track::$tracking>,
                for<'b> &'b mut ViewMut<'a, U>: Get<Out = $out>,
            {
                /// Delete if either component is not present or return `None`
                #[track_caller]
                #[allow(unused_mut)]
                pub fn update_or_delete<F>(self, mut update_fn: F)
                where
                    F: FnMut(EntityId, &T1, &T2) -> Option<U>,
                {
                    let UpdateTwoToOne(v_t1, v_t2, mut vm_u) = self;

                    let mut deleted_ids = HashSet::new();
                    deleted_ids.extend(v_t1.removed_or_deleted());
                    deleted_ids.extend(v_t2.removed_or_deleted());
                    for &e in &deleted_ids {
                        vm_u.delete(e);
                    }

                    let mut handled_ids = deleted_ids;

                    let mut inserted_ids = HashSet::new();
                    inserted_ids.extend(
                        v_t1.inserted()
                            .iter()
                            .ids()
                            .filter(|e| !handled_ids.contains(e)),
                    );
                    inserted_ids.extend(
                        v_t2.inserted()
                            .iter()
                            .ids()
                            .filter(|e| !handled_ids.contains(e)),
                    );

                    for &e in &inserted_ids {
                        if let Ok((t1, t2)) = (&v_t1, &v_t2).get(e) {
                            if let Some(update) = update_fn(e, t1, t2) {
                                vm_u.add_component_unchecked(e, update)
                            } else {
                                vm_u.delete(e);
                            }
                        }
                    }

                    handled_ids.extend(inserted_ids);

                    let mut modified_ids = HashSet::new();
                    modified_ids.extend(
                        v_t1.modified()
                            .iter()
                            .ids()
                            .filter(|e| !handled_ids.contains(e)),
                    );
                    modified_ids.extend(
                        v_t2.modified()
                            .iter()
                            .ids()
                            .filter(|e| !handled_ids.contains(e)),
                    );
                    for e in modified_ids {
                        if let Ok((t1, t2)) = (&v_t1, &v_t2).get(e) {
                            if let Some(update) = update_fn(e, t1, t2) {
                                // this is a weird way to write this
                                // the compile has trouble here if it's written with `if` `else`
                                if let Ok(mut exist) = (&mut vm_u).get(e) {
                                    if *exist != update {
                                        *exist = update;
                                    }
                                } else {
                                    vm_u.add_component_unchecked(e, update);
                                }
                            } else {
                                vm_u.delete(e);
                            }
                        }
                    }
                }
            }
        )+
    }
}

impl_update_two_to_one!(
    (Untracked, &'b mut U),
    (Insertion, &'b mut U),
    (Modification, Mut<'b, U>),
    (Deletion, &'b mut U),
    (Removal, &'b mut U),
    (All, Mut<'b, U>)
);
