use crate::loader::DecayLightLoader;
use crate::loader::LightTypeLoader;

use crate::prop_proxy_getters;

type PrimitiveLoader<T> = fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader<T>;
type RgbLoader<T> = fbxcel_dom::v7400::object::property::loaders::RgbLoader<T>;

pub struct LightProperties<'a> {
    prop_handle: fbxcel_dom::v7400::object::property::PropertiesHandle<'a>,
}

impl<'a> LightProperties<'a> {
    pub(crate) fn new(
        light_handle: &'a fbxcel_dom::v7400::object::nodeattribute::LightHandle,
    ) -> Option<Self> {
        let direct_light_properties = light_handle.direct_properties();

        direct_light_properties.map(|valid_light_properties| Self {
            prop_handle: valid_light_properties,
        })
    }

    prop_proxy_getters! {
        //Light Type
        light_type -> fabled_render::light::LightType{
            name = "LightType",
            loader = LightTypeLoader::default(),
            description = "light type",
            default : {
                light_type_or_default = fabled_render::light::LightType::DirectionalLight(fabled_render::light::DirectionalLight::default())
            }
        }

        cast_light -> bool{
            name = "CastLight",
            loader = PrimitiveLoader::<bool>::default(),
            description = "cast lighting",
            default : {
                cast_light_or_default = false
            }
        }

        light_color -> rgb::RGB<f64>{
            name = "Color",
            loader = RgbLoader::<rgb::RGB<f64>>::default(),
            description = "light color",
            default : {
                light_color_or_default = rgb::RGB::from([0.0; 3])
            }
        }

        light_intesity -> f32 {
            name = "Intensity",
            loader = PrimitiveLoader::<f32>::default(),
            description = "light intensity",
            default : {
                // The default value of the intensity is 100.0 in the fbx documentation.
                light_intensity_or_default = 100.0
            }
        }

        light_decay_type -> fabled_render::light::DecayType{
            name = "DecayType",
            loader = DecayLightLoader::default(),
            description = "light decay type",
            default : {
              light_decay_type_or_default = fabled_render::light::DecayType::None
            }
        }

        light_decay_start -> f32{
            name = "DecayStart",
            loader  = PrimitiveLoader::<f32>::default(),
            description = "light decay start distance",
            default : {
                light_decay_start_or_default = 2000.0
            }
        }

        cast_shadow -> bool{
            name = "CastShadows",
            loader = PrimitiveLoader::<bool>::default(),
            description = "light casts shadows",
            default : {
                cast_shadow_or_default = false
            }
        }

        shadow_color -> rgb::RGB<f64>{
            name = "ShadowColor",
            loader = RgbLoader::<rgb::RGB<f64>>::default(),
            description = "shadow color",
            default : {
                shadow_color_or_default = rgb::RGB::from([0.0; 3])
            }
        }

        outer_angle -> f32{
            name = "OuterAngle",
            loader = PrimitiveLoader::<f32>::default(),
            description = "point light outer angle",
            default : {
                outer_angle_or_default = 70.0
            }
        }

        inner_angle -> f32{
            name = "InnerAngle",
            loader = PrimitiveLoader::<f32>::default(),
            description = "point light inner angle",
            default : {
                inner_angle_or_default = 70.0
            }
        }
    }
}


impl<'a> std::ops::Deref for LightProperties<'a> {
    type Target = fbxcel_dom::v7400::object::property::PropertiesHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.prop_handle
    }
}
