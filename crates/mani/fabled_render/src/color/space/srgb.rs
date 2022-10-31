const SRGB_POW_TRANSFER_FUNCTION: f32 = 1.0 / 2.4;

const RCP_FALLOFF: f32 = 1.0 / 12.92;
const RCP_LINEAR_FALLOFF: f32 = 1.0 / 1.055;


pub fn s_rgb_to_linear(s_rgb: [f32; 3]) -> [f32; 3] {
    let mut srgb = s_rgb.to_owned();

    for (channel_index, &channel) in s_rgb.iter().enumerate() {
        if channel > 0.0031308 {
            srgb[channel_index] = ((channel + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            srgb[channel_index] = channel * RCP_FALLOFF;
        }
    }

    srgb
}

pub fn linear_to_s_rgb(s_rgb: [f32; 3]) -> [f32; 3] {
    let mut srgb = s_rgb.to_owned();

    for (channel_index, &channel) in s_rgb.iter().enumerate() {
        if channel > 0.0031308 {
            srgb[channel_index] = 1.055 * channel.powf(SRGB_POW_TRANSFER_FUNCTION) - 0.055;
        } else {
            srgb[channel_index] = 12.92 * channel;
        }
    }

    srgb
}
