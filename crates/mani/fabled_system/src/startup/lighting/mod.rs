use fabled_render::light::CascadeSplit;


pub fn construct_cascade_resource(primary_world: &shipyard::World) {
    primary_world.add_unique(CascadeSplit {
        splits: Default::default(),
    });

    // add the rest of the cascade parameter.
}
