#[allow(dead_code)]
pub struct Light {
    // Position Direction is relative to World Space.
    // if W = 1 then Spotlight and Point Light, else Directional light = 0
    position: [f32; 4],
    direction: [f32; 4],
    // Store diffuse and specular light color.
    color: [f32; 4],
    // half angle of the spotlight cone. if it is not a spotlight then make it to PI
    angle: f32,
    // how far away the light will reach. (attenuation constant). If lighting is directional light
    // then range is disregarded
    range: f32,
    // used to modulate the computed light contribution. 1.0 by default.
    intensity: f32,
    // check if light is enabled. if false then after the light culling the light will not show
    // even though it hasn't been culled
    enabled: bool,
    // light type mask: 0 = directional light, 1 = spotlight, 2 = point light
    light_type: u8,
}
