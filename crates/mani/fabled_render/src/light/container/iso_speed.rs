use crate::light::ISO_ARITHMETIC_STANDARD;

#[derive(Copy, Clone, Debug)]
pub struct ISOSpeed {
    pub arithmetic_speed: f32,
}

impl Default for ISOSpeed {
    fn default() -> Self {
        Self {
            arithmetic_speed: 100.0,
        }
    }
}

impl ISOSpeed {
    pub fn new(iso_speed: f32) -> Self {
        Self {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn new_standard(iso_speed: f32) -> Self {
        let mut iso_speed = iso_speed;

        if let false = ISO_ARITHMETIC_STANDARD.contains(&iso_speed) {
            iso_speed = 100.0
        };

        Self {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn is_standard(&self) -> bool {
        ISO_ARITHMETIC_STANDARD.contains(&self.arithmetic_speed)
    }
}
