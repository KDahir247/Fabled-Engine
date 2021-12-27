use std::any::{type_name, TypeId};

use crate::{
    app_builder::AppBuilder, type_names::TypeNames, AppWorkload, AppWorkloadInfo, Plugin,
    TypeIdBuckets,
};
use shipyard::*;
use tracing::trace_span;

#[allow(clippy::needless_doctest_main)]
/// Containers of app logic and data
pub struct App {
    pub world: World,
    pub(crate) type_names: TypeNames,
    workload_ids: TypeIdBuckets<()>,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> App {
        App::new_with_world(World::new())
    }
    pub fn new_with_world(world: World) -> App {
        let type_names = TypeNames::default();
        App {
            world,
            workload_ids: TypeIdBuckets::new("Count times workload plugin added", &type_names),
            type_names,
        }
    }

    #[track_caller]
    pub fn add_plugin_workload<P>(&mut self, plugin: P) -> AppWorkload
    where
        P: Plugin + 'static,
    {
        self.add_plugin_workload_with_info(plugin).0
    }

    #[track_caller]
    pub fn add_plugin_workload_with_info<P>(&mut self, plugin: P) -> (AppWorkload, AppWorkloadInfo)
    where
        P: Plugin + 'static,
    {
        let workload_name = type_name::<P>();
        let workload_type_id = TypeId::of::<P>();
        let span = trace_span!("add_plugin_workload_with_info", plugin = ?workload_name);
        let _span = span.enter();
        let name: std::borrow::Cow<'static, str> = match self.workload_ids.associate_type::<P>(()) {
            crate::AssociateResult { nth } if nth == 1 => workload_name.into(),
            crate::AssociateResult { nth } => format!("{}_{}", workload_name, nth).into(),
        };
        let mut builder = AppBuilder::new(&self);
        plugin.build(&mut builder);
        builder.finish_with_info_named(name, workload_type_id)
    }

    /// Runs default workload
    #[track_caller]
    pub fn update(&self) {
        let span = trace_span!("update");
        let _span = span.enter();
        self.world.run_default().unwrap();
    }

    #[track_caller]
    pub fn run<'s, B, R, S: shipyard::System<'s, (), B, R>>(&'s self, s: S) -> R {
        self.world.run(s).unwrap()
    }

    #[track_caller]
    pub fn run_with_data<'s, Data, B, R, S: shipyard::System<'s, (Data,), B, R>>(
        &'s self,
        s: S,
        data: Data,
    ) -> R {
        self.world.run_with_data(s, data).unwrap()
    }
}
