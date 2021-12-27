use std::{any::TypeId, borrow::Cow, collections::HashSet, sync::Arc};

use crate::{
    App, AppWorkload, AppWorkloadInfo, PluginAssociated, TypeIdBuckets, WorkloadSignature,
};

/// Associations made by this workload which includes the list of plugins and their reasons associated.
///
/// e.g. associated by requiring something to be update packed
#[derive(Clone)]
pub struct CycleWorkloadAssociations {
    workload: Cow<'static, str>,
    /// If this cycle workload is derived from a plugin, here's its [TypeId].
    workload_plugin_id: TypeId,
    /// List of plugins & their reasons for being associated
    plugins: Vec<PluginAssociated>,
}

impl std::fmt::Debug for CycleWorkloadAssociations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&self.workload)
            .field("plugins", &self.plugins)
            .finish()
    }
}

#[derive(Debug)]
pub enum CycleCheckError {
    UpdatePackResetInMultipleWorkloads {
        update_pack: &'static str,
        conflicts: Vec<CycleWorkloadAssociations>,
    },
    TrackedUniqueResetInMultipleWorkloads {
        tracked_unique: &'static str,
        conflicts: Vec<CycleWorkloadAssociations>,
    },
}

pub struct CycleSummary {
    cycle_order: Vec<Cow<'static, str>>,
    workload_info: Vec<CycleWorkloadSummary>,
}

pub struct CycleWorkloadSummary {
    name: Cow<'static, str>,
    signature: Arc<WorkloadSignature>,
}

impl std::fmt::Debug for CycleSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cycle")
            .field("order", &self.cycle_order)
            .field("workloads", &self.workload_info)
            .finish()
    }
}

impl std::fmt::Debug for CycleWorkloadSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&self.name)
            .field("update_packs", &self.signature.track_update_packed)
            .field("tracks_uniques", &self.signature.track_tracked_uniques)
            .finish()
    }
}

impl App {
    /// Check the ordering of these workloads to check for conflicts.
    ///
    /// Conflicts guarded against:
    ///  * Two different workloads require update_pack for the same storage
    pub fn add_cycle(
        &mut self,
        cycle: Vec<(AppWorkload, AppWorkloadInfo)>,
    ) -> Result<(AppWorkload, CycleSummary), Vec<CycleCheckError>> {
        // to track the plugins added so far (so we can avoid them accidentally conflicting with themselves)
        let mut workload_plugins_added = HashSet::new();
        let mut names_checked = Vec::new();
        let mut cumulative_update_packed = TypeIdBuckets::<CycleWorkloadAssociations>::new(
            "update packed in workloads",
            &self.type_names,
        );
        let mut cumulative_tracked_uniques = TypeIdBuckets::<CycleWorkloadAssociations>::new(
            "tracked uniques in workloads",
            &self.type_names,
        );

        let mut summary = CycleSummary {
            cycle_order: Vec::new(),
            workload_info: Vec::new(),
        };

        for (
            _workloads,
            AppWorkloadInfo {
                name,
                plugin_id,
                signature,
                batch_info: _,
                type_names: _,
            },
        ) in cycle
        {
            summary.cycle_order.push(name.clone());

            let is_unique_workload = if workload_plugins_added.contains(&plugin_id) {
                // If a cycle has the same workload multiple times, we don't want to get a
                // tracking conflict requirement with itself.
                false
            } else {
                // first seen plugin will be checked for tracking conflicts
                workload_plugins_added.insert(plugin_id);
                true
            };

            // We do not want to double count plugins so we can later check for conflicts
            if is_unique_workload {
                summary.workload_info.push(CycleWorkloadSummary {
                    name: name.clone(),
                    signature: signature.clone(),
                });

                // account for update packs
                for ((up_type, _), assoc) in signature.track_update_packed.entries() {
                    if !assoc.is_empty() {
                        // We need to account for these tracked_uniques
                        cumulative_update_packed.associate(
                            up_type,
                            CycleWorkloadAssociations {
                                plugins: assoc,
                                workload_plugin_id: plugin_id,
                                workload: name.clone(),
                            },
                        );
                    }
                }
                // account for tracked uniques
                for ((tracked_type, _), assoc) in signature.track_tracked_uniques.entries() {
                    if !assoc.is_empty() {
                        // We need to account for these tracked_uniques
                        cumulative_tracked_uniques.associate(
                            tracked_type,
                            CycleWorkloadAssociations {
                                plugins: assoc,
                                workload: name.clone(),
                                workload_plugin_id: plugin_id,
                            },
                        );
                    }
                }
            }

            names_checked.push(name.clone());
        }

        let mut errs = Vec::<CycleCheckError>::new();

        // update pack
        errs.extend(
            cumulative_update_packed
                .entries()
                .into_iter()
                .filter(|((_, _), workloads_dependent)| workloads_dependent.len() > 1)
                .map(|((_, update_pack_storage_name), workloads_dependent)| {
                    CycleCheckError::UpdatePackResetInMultipleWorkloads {
                        update_pack: update_pack_storage_name,
                        conflicts: workloads_dependent,
                    }
                }),
        );

        // tracked unique
        errs.extend(
            cumulative_tracked_uniques
                .entries()
                .into_iter()
                .filter(|((_, _), workloads_dependent)| workloads_dependent.len() > 1)
                .map(|((_, tracked_unique_storage_name), workloads_dependent)| {
                    CycleCheckError::TrackedUniqueResetInMultipleWorkloads {
                        tracked_unique: tracked_unique_storage_name,
                        conflicts: workloads_dependent,
                    }
                }),
        );

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok((
            AppWorkload {
                names: names_checked,
            },
            summary,
        ))
    }
}

#[cfg(test)]
mod update_pack_tests {
    use std::any::type_name;

    use super::*;
    use shipyard::Component;

    #[derive(Component)]
    #[track(All)]
    struct A;
    struct RxA1;
    struct RxA2;
    /// Can be added multiple times
    struct RxADup;
    struct RxTrackA1;
    struct RxTrackA2;
    /// Can be added multiple times
    struct RxTrackADup;
    struct OtherPlugin;

    fn setup_app() -> App {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".to_string()),
            ))
            .finish();
        let _ = ::tracing::subscriber::set_global_default(subscriber);

        App::new()
    }

    #[test]
    fn test_conflicting_update_packs() {
        let mut app = setup_app();

        // Given added workload 1 depends on [A] being update packed
        let rx_a1 = app.add_plugin_workload_with_info(RxA1);
        // And adding workload 2 depends on SAME update packed [A]
        let rx_a2 = app.add_plugin_workload_with_info(RxA2);

        // When declaring in a cycle
        let result = app.add_cycle(vec![rx_a1, rx_a2]);

        // Then observe an error
        let errors = result.expect_err("expected conflict");

        assert_eq!(
            errors.len(),
            1,
            "Expected 1 error, but found: {:#?}",
            errors
        );
        let one_err = errors.first().unwrap();
        if let CycleCheckError::UpdatePackResetInMultipleWorkloads { update_pack, .. } = one_err {
            assert_eq!(*update_pack, type_name::<A>());
        } else {
            panic!(
                "Expected error to be UpdatePackResetInMultipleWorkloads, but found {:#?}",
                one_err
            );
        }
    }

    #[test]
    fn test_duplicates_update_packed_ok() {
        let mut app = setup_app();

        // Given added workload 1 depends on [A] being update packed
        let rx_a_1 = app.add_plugin_workload_with_info(RxADup);
        // And adding the SAME workload 2
        let rx_a_2 = app.add_plugin_workload_with_info(RxADup);

        // When declaring in a cycle
        let result = app.add_cycle(vec![rx_a_1, rx_a_2]);

        // Then observe ok
        result.expect("expected no conflict");
    }

    #[test]
    fn test_conflicting_tracked() {
        let mut app = setup_app();

        // Given added workload 1 depends on [A] being a tracked unique
        let rx_a1 = app.add_plugin_workload_with_info(RxTrackA1);
        // And adding workload 2 depends on SAME tracked unique [A]
        let rx_a2 = app.add_plugin_workload_with_info(RxTrackA2);

        // When declaring in a cycle
        let result = app.add_cycle(vec![rx_a1, rx_a2]);

        // Then observe an error
        let errors = result.expect_err("expected conflict");

        assert_eq!(
            errors.len(),
            1,
            "Expected 1 error, but found: {:#?}",
            errors
        );
        let one_err = errors.first().unwrap();
        if let CycleCheckError::TrackedUniqueResetInMultipleWorkloads { tracked_unique, .. } =
            one_err
        {
            assert_eq!(*tracked_unique, type_name::<A>());
        } else {
            panic!(
                "Expected error to be TrackedUniqueResetInMultipleWorkloads, but found {:#?}",
                one_err
            );
        }
    }

    #[test]
    fn test_duplicates_tracked_ok() {
        let mut app = setup_app();

        // Given added workload 1 depends on [A] being tracked unique
        let rx_a_1 = app.add_plugin_workload_with_info(RxTrackADup);
        // And adding the SAME workload 2
        let rx_a_2 = app.add_plugin_workload_with_info(RxTrackADup);

        // When declaring in a cycle
        let result = app.add_cycle(vec![rx_a_1, rx_a_2]);

        // Then observe ok
        result.expect("expected no conflict");
    }

    impl crate::Plugin for RxA1 {
        fn build(&self, app: &mut crate::AppBuilder) {
            app.update_pack::<A>("RxA1");
        }
    }

    impl crate::Plugin for RxA2 {
        fn build(&self, app: &mut crate::AppBuilder) {
            app.update_pack::<A>("RxA2");
        }
    }

    impl crate::Plugin for RxADup {
        fn build(&self, app: &mut crate::AppBuilder) {
            app.update_pack::<A>("RxADup");
        }
        fn can_add_multiple_times(&self) -> bool {
            true
        }
    }

    impl crate::Plugin for OtherPlugin {
        fn build(&self, _: &mut crate::AppBuilder) {}
    }

    impl crate::Plugin for RxTrackA1 {
        fn build(&self, app: &mut crate::AppBuilder) {
            app.tracks::<A>("RxTrackA1");
        }
    }

    impl crate::Plugin for RxTrackA2 {
        fn build(&self, app: &mut crate::AppBuilder) {
            app.tracks::<A>("RxTrackA2");
        }
    }
    impl crate::Plugin for RxTrackADup {
        fn build(&self, app: &mut crate::AppBuilder) {
            app.tracks::<A>("RxTrackADup");
        }
        fn can_add_multiple_times(&self) -> bool {
            true
        }
    }
}
