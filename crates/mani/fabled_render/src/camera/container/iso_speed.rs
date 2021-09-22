use crate::camera::{logarithmic_to_arithmetic_speed, ISOSpeedUnit, ISO_ARITHMETIC_STANDARD};

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
    pub fn new(iso_speed: f32, unit_type: ISOSpeedUnit) -> Self {
        let mut iso_speed = iso_speed;

        if let ISOSpeedUnit::Logarithmic = unit_type {
            iso_speed = logarithmic_to_arithmetic_speed(iso_speed);
        };

        Self {
            arithmetic_speed: iso_speed,
        }
    }

    pub fn new_standard(iso_speed: f32, unit_type: ISOSpeedUnit) -> Self {
        let mut iso_speed = iso_speed;


        if let ISOSpeedUnit::Logarithmic = unit_type {
            iso_speed = logarithmic_to_arithmetic_speed(iso_speed);
        }

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


#[cfg(test)]
mod iso_test {
    use crate::camera::{arithmetic_to_logarithmic_speed, ISOSpeed, ISOSpeedUnit};

    #[test]
    fn creation_iso() {
        let iso = ISOSpeed::new(100.0, ISOSpeedUnit::Arithmetic);

        let log_representation = arithmetic_to_logarithmic_speed(iso.arithmetic_speed);

        let iso_log = ISOSpeed::new(42.0, ISOSpeedUnit::Logarithmic);
        let same_iso = ISOSpeed::new(log_representation, ISOSpeedUnit::Logarithmic);

        assert!(iso.arithmetic_speed.ne(&iso_log.arithmetic_speed));
        println!("{}, {}", iso.arithmetic_speed, iso_log.arithmetic_speed);

        assert!(iso.arithmetic_speed.eq(&same_iso.arithmetic_speed));
        println!("{}, {}", iso.arithmetic_speed, same_iso.arithmetic_speed)
    }


    #[test]
    fn creation_standard_iso() {
        let iso_standard = ISOSpeed::new_standard(160.0, ISOSpeedUnit::Arithmetic);
        assert!(iso_standard.is_standard());
        println!("{}", iso_standard.arithmetic_speed);

        let iso_standard = ISOSpeed::new_standard(100.0, ISOSpeedUnit::Logarithmic);
        assert!(iso_standard.is_standard());
        println!("{}", iso_standard.arithmetic_speed);
    }
}
