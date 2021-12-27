use crate::{
    app::App, plugin::Plugin, tracked_unique::reset_tracked_unique, type_names::TypeNames,
};
use shipyard::*;
use std::{
    any::{type_name, TypeId},
    borrow::Cow,
    collections::hash_map::Entry,
    collections::HashMap,
    sync::Arc,
};
use tracing::*;

mod plugin_id;
use plugin_id::PluginId;

/// Used when a workload is created without a plugin
pub static DEFAULT_WORKLOAD_NAME: &str = "update";

/// Used for defaultly created workloads
pub struct DefaultWorkloadPlugin;

#[derive(Clone)]
pub struct PluginAssociated {
    pub plugin: PluginId,
    pub reason: &'static str,
}

impl std::fmt::Debug for PluginAssociated {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // shorten plugin name for printing
        let plugin_name_short = format!("{:?}", self.plugin)
            .split("::")
            .last()
            .unwrap_or("<Error: Plugin has no names>")
            .replace(")", "");
        write!(&mut f, "{}: {}", &plugin_name_short, &self.reason,)
    }
}

pub(crate) type PluginsAssociatedMap = TypeIdBuckets<PluginAssociated>;

pub(crate) struct TypeIdBuckets<T> {
    pub(crate) name: &'static str,
    pub(crate) track_type_names: TypeNames,
    pub(crate) type_plugins_lookup: HashMap<TypeId, Vec<T>>,
}

impl<T: std::fmt::Debug + Clone> std::fmt::Debug for TypeIdBuckets<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"association", &self.name)
            .entries(
                self.entries()
                    .into_iter()
                    .map(|((_, name), associated)| (name, associated)),
            )
            .finish()
    }
}

pub struct AssociateResult {
    pub(crate) nth: usize,
}

impl AssociateResult {
    fn is_first(&self) -> bool {
        self.nth == 1
    }
}

impl<T> TypeIdBuckets<T> {
    pub(crate) fn new(name: &'static str, track_type_names: &TypeNames) -> Self {
        TypeIdBuckets {
            name,
            track_type_names: track_type_names.clone(),
            type_plugins_lookup: Default::default(),
        }
    }
}

impl<T: Clone + std::fmt::Debug> TypeIdBuckets<T> {
    pub(crate) fn entries(&self) -> Vec<((TypeId, &'static str), Vec<T>)> {
        let mut ordered = self
            .type_plugins_lookup
            .iter()
            .map(|(id, associations)| -> ((TypeId, &'static str), Vec<T>) {
                let name = self
                    .track_type_names
                    .lookup_name(&id)
                    .expect("all type ids with associations have a name saved");
                ((*id, name), associations.to_vec())
            })
            .collect::<Vec<_>>();

        ordered.sort_by_key(|a| a.0 .1);

        ordered
    }

    pub(crate) fn associate_type<U: 'static>(&mut self, assoc: T) -> AssociateResult {
        self.associate(self.track_type_names.tracked_type_id_of::<U>(), assoc)
    }

    pub(crate) fn associate(&mut self, type_id: TypeId, assoc: T) -> AssociateResult {
        let type_name = self.track_type_names.lookup_name(&type_id).unwrap();
        match self.type_plugins_lookup.entry(type_id) {
            Entry::Occupied(mut list) => {
                trace!(
                    ?assoc,
                    with = type_name,
                    result = "existed",
                    "Associated {}",
                    self.name
                );
                // no need to pack again
                list.get_mut().push(assoc);
                AssociateResult {
                    nth: list.get().len(),
                }
            }
            Entry::Vacant(list) => {
                trace!(
                    ?assoc,
                    with = type_name,
                    result = "added",
                    "Associated {}",
                    self.name
                );
                list.insert(vec![assoc]);
                AssociateResult { nth: 1 }
            }
        }
    }
}

impl TypeIdBuckets<PluginAssociated> {
    /// Return new number of plugins associated
    pub(crate) fn associate_plugin<Type: Component>(
        &mut self,
        plugin: &PluginId,
        reason: &'static str,
    ) -> AssociateResult {
        let type_id = self.track_type_names.tracked_type_id_of::<Type>();
        let assoc = PluginAssociated {
            plugin: plugin.clone(),
            reason,
        };
        self.associate(type_id, assoc)
    }
}

#[derive(Debug)]
pub(crate) struct WorkloadSignature {
    /// track the plugins directly required by other plugins
    pub track_plugin_dependencies: PluginsAssociatedMap,
    /// unique type id to list of plugin type ids that provided a value for it it
    pub track_uniques_provided: PluginsAssociatedMap,
    pub track_tracked_uniques_provided: PluginsAssociatedMap,
    /// unique type id to list of (plugin type id, reason string)
    pub track_unique_dependencies: PluginsAssociatedMap,
    /// update component storage type id to list of (plugin type id, reason string)
    pub track_update_packed: PluginsAssociatedMap,
    /// tracked uniques storage type id to list of (plugin type id, reason string)
    pub track_tracked_uniques: PluginsAssociatedMap,
}

impl WorkloadSignature {
    pub(crate) fn new(type_names: &TypeNames) -> Self {
        WorkloadSignature {
            track_plugin_dependencies: PluginsAssociatedMap::new(
                "Plugin depends on Plugin",
                &type_names,
            ),
            track_uniques_provided: PluginsAssociatedMap::new(
                "Plugin provides Unique",
                &type_names,
            ),
            track_tracked_uniques_provided: PluginsAssociatedMap::new(
                "Plugin provides Tracked unique",
                &type_names,
            ),
            track_unique_dependencies: PluginsAssociatedMap::new(
                "Plugin depends on Unique",
                &type_names,
            ),
            track_update_packed: PluginsAssociatedMap::new(
                "Plugin requires update_pack",
                &type_names,
            ),
            track_tracked_uniques: PluginsAssociatedMap::new(
                "Plugin requires tracked unique",
                &type_names,
            ),
        }
    }
}

/// Configure [App]s using the builder pattern
pub struct AppBuilder<'a> {
    pub app: &'a App,
    resets: Vec<WorkloadSystem>,
    systems: Vec<WorkloadSystem>,
    /// track the plugins previously added to enable checking that plugin peer dependencies are satisified
    track_added_plugins: HashMap<TypeId, PluginId>,
    /// track the currently being used plugin ([PluginId] is a stack since some plugins add other plugins creating a nest)
    // TODO: Track "Plugin"s for each thing
    track_current_plugin: PluginId,
    /// take a record of type names as we come across them for diagnostics
    track_type_names: TypeNames,
    signature: WorkloadSignature,
}

impl<'a> AppBuilder<'a> {
    pub fn new(app: &App) -> AppBuilder<'_> {
        AppBuilder::empty(app)
    }
}

#[derive(Clone, Debug)]
pub struct AppWorkload {
    pub(crate) names: Vec<std::borrow::Cow<'static, str>>,
}

#[derive(Clone, Debug)]
pub struct AppWorkloadInfo {
    pub(crate) type_names: Blind<TypeNames>,
    pub(crate) batch_info: Vec<info::BatchInfo>,
    /// Self-imposed constraints declared by the workload
    pub(crate) signature: Arc<WorkloadSignature>,
    /// Derived from this plugin
    pub(crate) plugin_id: TypeId,
    /// Workload name assigned in the world
    pub name: Cow<'static, str>,
}

#[derive(Clone)]
pub(crate) struct Blind<T: Clone + 'static>(pub T);

impl<T: Clone + 'static> std::fmt::Debug for Blind<T> {
    fn fmt(&self, mut f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(&mut f, "Blind<{}>", type_name::<T>())
    }
}

impl AppWorkload {
    #[track_caller]
    #[instrument(skip(app))]
    pub fn run(&self, app: &App) {
        for workload_name in self.names.iter() {
            let span = trace_span!("AppWorkload::run", ?workload_name);
            let _span = span.enter();
            app.world.run_workload(&workload_name).unwrap();
        }
    }
}

impl<'a> AppBuilder<'a> {
    /// The general approach to running a Shipyard App is to create a new shipyard [World],
    /// then pass that world into [App::build]. Then, after adding your plugins, you can call this [AppBuilder::finish] to get an [App].
    ///
    /// With this App, you can:
    ///  1. Update any Uniques first or use [World::run_with_data] to prime the rest of the systems, then
    ///  2. Call the [App::update()] function, and
    ///  3. Pull any data you need out from the [World], and repeat.
    ///
    /// # Panics
    /// May panic if there are unmet unique dependencies or if there is an error adding workloads to shipyard.
    #[track_caller]
    pub fn finish(self) -> AppWorkload {
        self.finish_with_info().0
    }

    /// Finish [App] and report back each of the update stages with their [AppWorkloadInfo].
    #[track_caller]
    fn finish_with_info(self) -> (AppWorkload, AppWorkloadInfo) {
        self.finish_with_info_named(
            DEFAULT_WORKLOAD_NAME.into(),
            std::any::TypeId::of::<DefaultWorkloadPlugin>(),
        )
    }

    /// Finish [App] and report back each of the update stages with their [AppWorkloadInfo].
    #[track_caller]
    #[instrument(skip(self))]
    pub(crate) fn finish_with_info_named(
        self,
        update_stage: std::borrow::Cow<'static, str>,
        plugin_id: TypeId,
    ) -> (AppWorkload, AppWorkloadInfo) {
        let AppBuilder {
            app,
            resets,
            systems,
            track_added_plugins: _,
            track_current_plugin: _,
            track_type_names: _,
            signature,
        } = self;

        let mut update_workload = systems.into_iter().fold(
            WorkloadBuilder::new(update_stage.clone()),
            |acc: WorkloadBuilder, system: WorkloadSystem| acc.with_system(system),
        );

        for reset_system in resets {
            update_workload = update_workload.with_system(reset_system);
        }

        let info = update_workload.add_to_world(&app.world).unwrap();
        (
            AppWorkload {
                names: vec![update_stage],
                // signature: Rc::new(signature),
            },
            AppWorkloadInfo {
                batch_info: info.batch_info,
                type_names: Blind(app.type_names.clone()),
                plugin_id,
                name: info.name,
                signature: Arc::new(signature),
            },
        )
    }

    /// Lookup the type id while simultaneously storing the type name to be referenced later
    fn tracked_type_id_of<T: 'static>(&mut self) -> TypeId {
        self.track_type_names.tracked_type_id_of::<T>()
    }

    /// Update component `T`'s storage to be update_pack, and add [shipyard::sparse_set::SparseSet::clear_all_inserted_and_modified] as the last system.
    #[track_caller]
    pub fn update_pack<T: Component<Tracking = track::All> + Send + Sync>(
        &mut self,
        reason: &'static str,
    ) -> &mut Self {
        if self
            .signature
            .track_update_packed
            .associate_plugin::<T>(&self.track_current_plugin, reason)
            .is_first()
        {
            self.app.world.borrow::<ViewMut<T>>().unwrap();
            self.resets.push(
                reset_update_pack::<T>
                    .into_workload_system()
                    .expect("system to be valid"),
            );
        }

        self
    }

    /// Declare dependency on `[crate::Tracked]<T>` and add "reset tracked" as the last system.
    #[track_caller]
    pub fn tracks<T: Component<Tracking = track::All> + Send + Sync>(
        &mut self,
        reason: &'static str,
    ) -> &mut Self {
        if self
            .signature
            .track_tracked_uniques
            .associate_plugin::<T>(&self.track_current_plugin, reason)
            .is_first()
        {
            self.resets.push(
                reset_tracked_unique::<T>
                    .into_workload_system()
                    .expect("system to be valid"),
            );
        }

        self
    }

    /// Add a unique component
    #[track_caller]
    pub fn add_unique<T: Component>(&mut self, component: T) -> &mut Self
    where
        T: Send + Sync + 'static,
    {
        if self
            .signature
            .track_uniques_provided
            .associate_plugin::<T>(&self.track_current_plugin, "<not provided>")
            .is_first()
        {
            self.app.world.add_unique(component).unwrap();
        } else {
            warn!(
                "Unique({}) already provided by another plugin in the plugin workload.",
                type_name::<T>(),
            )
        }

        self
    }

    /// Add a tracked unique value.
    ///
    /// Accessible through the [crate::Tracked] and [crate::TrackedMut] views.
    #[track_caller]
    pub fn add_tracked_value<T: Component<Tracking = track::All>>(
        &mut self,
        component: T,
    ) -> &mut Self
    where
        T: Send + Sync + 'static,
    {
        if self
            .signature
            .track_tracked_uniques_provided
            .associate_plugin::<T>(&self.track_current_plugin, "<not provided>")
            .is_first()
        {
            self.app.world.add_unique(component).unwrap();
        } else {
            warn!(
                "Tracked unique({}) already provided by another plugin in the plugin workload.",
                type_name::<T>(),
            )
        }

        self
    }

    /// Declare that this builder has a dependency on the following unique.
    ///
    /// If the unique dependency is not satisfied by the time [AppBuilder::finish] is called, then the finish call will panic.
    #[track_caller]
    pub fn depends_on_unique<T>(&mut self, dependency_reason: &'static str) -> &mut Self
    where
        T: Send + Sync + Component,
    {
        self.signature
            .track_unique_dependencies
            .associate_plugin::<T>(&self.track_current_plugin, dependency_reason);

        self
    }

    /// Declare that this builder has a dependency on the following plugin.
    #[track_caller]
    pub fn depends_on_plugin<T>(&mut self, dependency_reason: &'static str) -> &mut Self
    where
        T: Plugin,
    {
        let plugin_type_id = self.tracked_type_id_of::<T>();
        if !self.track_added_plugins.contains_key(&plugin_type_id) {
            panic!(
                "\"{}\" depends on \"{}\": {}",
                self.track_current_plugin,
                type_name::<T>(),
                dependency_reason
            );
        }
        self
    }

    fn empty(app: &App) -> AppBuilder<'_> {
        AppBuilder {
            app,
            resets: Vec::new(),
            systems: Vec::new(),
            track_added_plugins: Default::default(),
            track_current_plugin: Default::default(),
            track_type_names: Default::default(),
            signature: WorkloadSignature::new(&app.type_names),
        }
    }

    #[track_caller]
    pub fn add_system<B, R, S: IntoWorkloadSystem<B, R>>(&mut self, system: S) -> &mut Self {
        self.systems
            .push(system.into_workload_system().expect("system to be valid"));

        self
    }

    /// Ensure that this system is among the absolute last systems
    #[track_caller]
    pub fn add_reset_system<B, R, S: IntoWorkloadSystem<B, R>>(
        &mut self,
        system: S,
        reason: &str,
    ) -> &mut Self {
        trace!(plugin = ?self.track_current_plugin, ?reason, "add_reset_system");
        self.resets
            .push(system.into_workload_system().expect("system to be valid"));

        self
    }

    #[track_caller]
    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        let plugin_type_id = self.tracked_type_id_of::<T>();
        let span = trace_span!("add_plugin", plugin = ?self.track_current_plugin, adding = ?type_name::<T>());
        let _span = span.enter();
        if let Some(plugin_id) = self.track_added_plugins.get(&plugin_type_id) {
            if !plugin.can_add_multiple_times() {
                panic!(
                    "Plugin ({}) cannot add plugin as it's already added as \"{}\". (Implement `Plugin::can_add_multiple_times` to override)",
                    self.track_current_plugin, plugin_id
                );
            }
        }

        if self.track_current_plugin.contains(plugin_type_id) {
            panic!(
                "Plugin ({}) cannot add plugin ({}) as it would cause a cycle",
                self.track_current_plugin,
                self.track_type_names
                    .lookup_name(&plugin_type_id)
                    .unwrap_or(""),
            );
        }

        self.track_current_plugin.push::<T>();
        trace_span!("build", plugin = ?self.track_current_plugin).in_scope(|| {
            plugin.build(self);
        });
        self.track_added_plugins
            .insert(plugin_type_id, self.track_current_plugin.clone());
        self.track_current_plugin.pop();
        self
    }
}

fn reset_update_pack<T: Component<Tracking = track::All>>(mut vm_to_clear: ViewMut<T>) {
    trace_span!("reset_update_pack", storage_name = type_name::<T>()).in_scope(|| {
        vm_to_clear.clear_all_inserted_and_modified();
        vm_to_clear.take_removed_and_deleted();
    });
}
