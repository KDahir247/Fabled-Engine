use crate::light::{DirectionalLight, PointLight, SpotLight};

#[derive(Clone, Debug)]
pub enum LightType {
    SpotLight(SpotLight),
    PointLight(PointLight),
    DirectionalLight(DirectionalLight),
}
