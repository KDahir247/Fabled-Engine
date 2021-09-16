use crate::light::{DirectionalLight, PointLight, SpotLight};

pub enum LightType {
    SpotLight(SpotLight),
    PointLight(PointLight),
    DirectionalLight(DirectionalLight),
}
