use super::setup;
use lib::component::prelude::*;

pub struct Graphic;

impl Graphic {
    pub fn run(world: &shipyard::World) -> anyhow::Result<()> {
        futures::executor::block_on(setup::run(world))?;
        Self::start(world)
    }
}

impl Graphic {
    fn start(world: &shipyard::World) -> anyhow::Result<()> {
        superluminal_perf::begin_event("Create_Entity");

        let (mut entities, render) =
            world.borrow::<(shipyard::EntitiesViewMut, shipyard::UniqueView<RenderData>)>()?;

        let lighting = LightRaw {
            position: glam::vec3(0.1, 0.3, 0.1),
            __padding__: 0,
            color: LightColorRaw {
                ambient_color: glam::vec4(1.0, 1.0, 1.0, 1.0),
                diffuse_color: glam::vec4(1.0, 1.0, 1.0, 1.0),
                specular_color: glam::vec4(1.0, 0.0, 0.0, 1.0),
                emissive_color: Default::default(),
            },
        };

        let lighting_uniform = LightUniform::create(&render.core.device, lighting);

        let uniform_storage = world.borrow::<shipyard::ViewMut<LightUniform>>()?;

        entities.add_entity(uniform_storage, lighting_uniform);

        superluminal_perf::end_event();

        Ok(())
    }
}
