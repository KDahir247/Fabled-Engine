mod add_distinct;
mod app;
mod app_add_cycle;
mod app_builder;
mod plugin;
mod tracked_unique;
mod type_names;
mod update_one_to_one;
mod update_two_to_one;

pub use add_distinct::*;
pub use app::*;
pub use app_builder::*;
pub use plugin::*;
pub use shipyard::*;
pub use tracked_unique::*;
pub use update_one_to_one::*;
pub use update_two_to_one::*;

pub use app_add_cycle::CycleSummary;

pub mod prelude {
    pub use crate::{
        add_distinct::AddDistinct,
        app::App,
        app_builder::{AppBuilder, AppWorkload},
        plugin::Plugin,
        update_one_to_one::UpdateOneToOne,
        update_two_to_one::UpdateTwoToOne,
    };
    pub use shipyard::*;
}

#[cfg(test)]
mod test {
    mod tree;
}
