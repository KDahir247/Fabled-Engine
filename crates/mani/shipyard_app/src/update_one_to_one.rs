use shipyard::*;

/// # Update one to one
///
/// A shipyard view for when you need to assign some component
/// based upon another component.
///
/// This is called "one-to-one", because you cannot optionally assign the
/// "write" component. If there is a "read" component changed, the "write"
/// component must have a value to be assigned.
///
/// Automatically manages checking for distinct values (if the "write" component
/// == its previous value, then no update). And, this view manages removing the
/// "write" component if the "read" component is removed.
///
/// ```
/// use shipyard_app::prelude::*;
///
/// /// Square is a one-to-one value determined from the [u32] components changes.
/// #[derive(Clone, Debug, PartialEq, Eq, Component)]
/// struct Square(u64);
/// #[derive(Component, Clone, Debug, PartialEq, Eq)]
/// #[track(All)]
/// struct U32(u32);
///
/// fn updating_squares<'a>(u32_to_square: UpdateOneToOne<'a, U32, Square>) {
///     u32_to_square.update(|_entity_id, u32_component| {
///         Square((u32_component.0 as u64) * (u32_component.0 as u64))
///     });
/// }
///
/// fn collecting_squares(v_u32: View<U32>, v_square: View<Square>) -> Vec<(U32, Square)> {
///     let mut res = (&v_u32, &v_square).iter().map(|(u, s)| (u.clone(), s.clone())).collect::<Vec<(U32, Square)>>();
///     res.sort_unstable_by_key(|(u, _)| u.0);
///     res
/// }
///
/// let mut world = World::new();
///
/// let entity_1 = world.add_entity((U32(1),));
/// world.add_entity((U32(2),));
/// world.add_entity((U32(3),));
///
/// world.run(updating_squares).unwrap();
///
/// assert_eq!(
///     world.run(collecting_squares).unwrap(),
///     vec![(U32(1), Square(1)), (U32(2), Square(4)), (U32(3), Square(9))]
/// );
///
/// // remove
/// world.run(|mut vm_u32: ViewMut<U32>| {
///     vm_u32.remove(entity_1).unwrap();
/// }).unwrap();
///
/// world.run(updating_squares).unwrap();
///
/// assert_eq!(
///     world.run(collecting_squares).unwrap(),
///     vec![(U32(2), Square(4)), (U32(3), Square(9))]
/// );
///
/// // entity_1 was removed from the [Square] storage as well
/// world.borrow::<View<Square>>()
///     .unwrap()
///     .get(entity_1)
///     .expect_err("expect Square removed in one to one");
/// ```
pub struct UpdateOneToOne<
    'a,
    T: Component<Tracking = track::All>,
    U: PartialEq + Component,
    UTrack: track::Tracking<U> = <U as Component>::Tracking,
>(View<'a, T>, ViewMut<'a, U, UTrack>);

pub struct UpdateOneToOneBorrower<T, U>(T, U);

impl<T, U> IntoBorrow for UpdateOneToOne<'_, T, U>
where
    T: Send + Sync + Component<Tracking = track::All>,
    U: PartialEq + Send + Sync + Component,
    <U::Tracking as track::Tracking<U>>::DeletionData: Send + Sync,
{
    type Borrow = UpdateOneToOneBorrower<T, U>;
}

impl<'a, T, U> Borrow<'a> for UpdateOneToOneBorrower<T, U>
where
    T: Send + Sync + Component<Tracking = track::All>,
    U: PartialEq + Send + Sync + Component,
    <U::Tracking as track::Tracking<U>>::DeletionData: Send + Sync,
{
    type View = UpdateOneToOne<'a, T, U>;

    fn borrow(world: &'a World) -> Result<Self::View, error::GetStorage> {
        Ok(UpdateOneToOne(world.borrow()?, world.borrow()?))
    }
}

unsafe impl<
        'a,
        T: Component<Tracking = track::All> + Send + Sync,
        U: Component + Send + Sync + PartialEq,
    > BorrowInfo for UpdateOneToOne<'a, T, U>
{
    fn borrow_info(mut info: &mut Vec<info::TypeInfo>) {
        View::<'a, T>::borrow_info(&mut info);
        ViewMut::<'a, U>::borrow_info(&mut info);
    }
}

// The compiler has trouble abstracting the types with a bound, so I use a macro
// The impl is the same for all tracking
// The only change is the type returned, `&mut U` if no modification tracking, `Mut<U>` otherwise
macro_rules! impl_update_one_to_one {
    ($(($tracking: ident, $out: ty)),+) => {
        $(
            impl<'a, T, U> UpdateOneToOne<'a, T, U, track::$tracking>
            where
                T: Sync + Send + Component<Tracking = track::All>,
                U: PartialEq + Component<Tracking = track::$tracking>,
                for<'b> &'b mut ViewMut<'a, U>: Get<Out = $out>,
            {
                #[track_caller]
                pub fn update<F>(self, mut update_fn: F)
                where
                    F: FnMut(EntityId, &T) -> U,
                {
                    self.update_or_ignore(move |e, t| Some(update_fn(e, t)))
                }

                #[track_caller]
                #[allow(unused_mut)]
                pub fn update_or_ignore<F>(self, mut update_fn: F)
                where
                    F: FnMut(EntityId, &T) -> Option<U>,
                {
                    let UpdateOneToOne(v_t, mut vm_u) = self;
                    for (e, t) in v_t.inserted().iter().with_id() {
                        if let Some(update) = update_fn(e, t) {
                            vm_u.add_component_unchecked(e, update)
                        }
                    }
                    for (e, t) in v_t.modified().iter().with_id() {
                        if let Some(update) = update_fn(e, t) {
                            if let Ok(mut exist) = (&mut vm_u).get(e) {
                                if *exist != update {
                                    *exist = update;
                                }
                            } else {
                                vm_u.add_component_unchecked(e, update);
                            }
                        }
                    }
                    for e in v_t.removed_or_deleted() {
                        vm_u.delete(e);
                    }
                }

                #[track_caller]
                #[allow(unused_mut)]
                pub fn update_or_delete<F>(self, mut update_fn: F)
                where
                    F: FnMut(EntityId, &T) -> Option<U>,
                {
                    let UpdateOneToOne(v_t, mut vm_u) = self;
                    for (e, t) in v_t.inserted().iter().with_id() {
                        if let Some(update) = update_fn(e, t) {
                            vm_u.add_component_unchecked(e, update)
                        } else {
                            vm_u.delete(e);
                        }
                    }
                    for (e, t) in v_t.modified().iter().with_id() {
                        if let Some(update) = update_fn(e, t) {
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
                    for e in v_t.removed_or_deleted() {
                        vm_u.delete(e);
                    }
                }
            }
        )+
    };
}

impl_update_one_to_one!(
    (Untracked, &'b mut U),
    (Insertion, &'b mut U),
    (Modification, Mut<'b, U>),
    (Deletion, &'b mut U),
    (Removal, &'b mut U),
    (All, Mut<'b, U>)
);
