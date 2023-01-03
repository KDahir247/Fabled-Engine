use crate::light::component::PointLight;
use crate::light::Source;
use fabled_component::{All, Component};
use fabled_math::Vector3;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroU32;

struct CubeMapFace {
    view: Vector3,
    up: Vector3,
}

impl CubeMapFace {
    const POSITIVE_X: CubeMapFace = CubeMapFace {
        view: Vector3::RIGHT,
        up: Vector3::DOWN,
    };

    const NEGATIVE_X: CubeMapFace = CubeMapFace {
        view: Vector3::LEFT,
        up: Vector3::DOWN,
    };

    const POSITIVE_Y: CubeMapFace = CubeMapFace {
        view: Vector3::UP,
        up: Vector3::FORWARD,
    };

    const NEGATIVE_Y: CubeMapFace = CubeMapFace {
        view: Vector3::DOWN,
        up: Vector3::BACKWARD,
    };

    const POSITIVE_Z: CubeMapFace = CubeMapFace {
        view: Vector3::FORWARD,
        up: Vector3::DOWN,
    };

    const NEGATIVE_Z: CubeMapFace = CubeMapFace {
        view: Vector3::BACKWARD,
        up: Vector3::DOWN,
    };
}


#[derive(Copy, Clone)]
pub struct ShadowMapper<T: Source> {
    pub cascade: Option<NonZeroU32>,
    pub depth_bias: f32,
    pub normal_bias: f32,
    pub resolution: f32,
    phantom_data: PhantomData<*const T>,
}


impl<T: Source> Default for ShadowMapper<T> {
    fn default() -> Self {
        ShadowMapper {
            cascade: None,
            depth_bias: 0.02,
            normal_bias: 0.6,
            resolution: 4096.0,
            phantom_data: Default::default(),
        }
    }
}

impl ShadowMapper<PointLight> {
    pub const fn compute_shadow_cube_map() -> [CubeMapFace; 6] {
        [
            CubeMapFace::POSITIVE_X,
            CubeMapFace::NEGATIVE_X,
            CubeMapFace::POSITIVE_Y,
            CubeMapFace::NEGATIVE_Y,
            CubeMapFace::POSITIVE_Z,
            CubeMapFace::NEGATIVE_Z,
        ]
    }
}

impl<T: Source> Display for ShadowMapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "ShadowMapper(\n\tcascades : {:?}\n\tdepth bias : {},\n\tnormal bias : {}\n\tresolution : {}\n)",
            self.cascade,self.depth_bias, self.normal_bias, self.resolution
        )
    }
}

impl<T: Source + 'static> Component for ShadowMapper<T> {
    type Tracking = All;
}
