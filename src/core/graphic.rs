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
        let (mut entities, setup) =
            world.borrow::<(shipyard::EntitiesViewMut, shipyard::UniqueView<Setup>)>()?;

        let lighting = DirectionalLightRaw {
            direction: glam::vec3(0.5, 0.0, 0.0),
            __padding__: 0,
            color: LightColorRaw {
                ambient_color: glam::vec4(1.0, 1.0, 1.0, 0.4),
                diffuse_color: glam::vec4(1.0, 0.0, 0.0, 1.0),
                specular_color: glam::vec4(1.0, 0.0, 0.0, 1.0),
            },
        };

        let lighting_uniform = LightUniform::create(&setup.device, lighting);

        let (uniform_storage) = world.borrow::<shipyard::ViewMut<LightUniform>>()?;

        entities.add_entity(uniform_storage, lighting_uniform);

        Ok(())
    }
}
