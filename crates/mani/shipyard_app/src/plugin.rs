use std::any::{type_name, Any};

use crate::AppBuilder;

/// A collection of Bevy App logic and configuration
///
/// Plugins use [AppBuilder] to configure an [App](crate::App). When an [App](crate::App) registers a plugin, the plugin's [Plugin::build] function is run.
pub trait Plugin: Any + Send + Sync {
    fn build(&self, app: &mut AppBuilder);
    fn name(&self) -> &str {
        type_name::<Self>()
    }
    /// If you override this to return true, then you will be allowed to add your plugin multiple times.
    fn can_add_multiple_times(&self) -> bool {
        false
    }
}
