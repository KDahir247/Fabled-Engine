//! A small change tracker to wrap around a unique so you can determine if the value changes between updates.
use tracing::trace_span;

use crate::prelude::*;

use core::any::type_name;

/// Add [TrackedUnique] of `T` and reset tracking at the end of every update.
#[derive(Default)]
pub struct TrackedUniquePlugin<T: Clone + Send + Sync + Component<Tracking = track::All>>(T);

impl<T: Clone + Send + Sync + Component<Tracking = track::All>> TrackedUniquePlugin<T> {
    pub fn new(initial_value: T) -> Self {
        TrackedUniquePlugin(initial_value)
    }
}

impl<T: Clone + Send + Sync + Component<Tracking = track::All>> Plugin for TrackedUniquePlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_unique(self.0.clone());
    }
}

pub(crate) fn reset_tracked_unique<T: Component<Tracking = track::All>>(
    mut uvm_tracked_unique_t: UniqueViewMut<T>,
) {
    let span = trace_span!("reset_tracked_unique", tracked = ?type_name::<T>());
    let _span = span.enter();
    uvm_tracked_unique_t.clear_inserted_and_modified();
}
