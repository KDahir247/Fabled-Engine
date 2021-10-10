// vertical_position get the vertical position relative to angle_rad
// depth_offset get the z position relative to angle_rad

#[repr(align(8))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Oblique {
    pub angle_rad: f32,
    pub vertical_position: f32,
    pub depth_offset: f32,
}

impl Default for Oblique {
    fn default() -> Self {
        Self {
            angle_rad: std::f32::consts::FRAC_PI_4,
            vertical_position: 0.05,
            depth_offset: 10.0,
        }
    }
}

impl Oblique {
    pub fn new(angle_degree: f32, vertical_position: f32, depth_offset: f32) -> Self {
        let angle = angle_degree.clamp(-180.0, 180.0);
        let vertical = vertical_position.clamp(-1.0, 1.0);

        Self {
            angle_rad: angle.to_radians(),
            vertical_position: vertical,
            depth_offset,
        }
    }
}


#[cfg(test)]
mod oblique_test {
    use crate::camera::Oblique;

    #[test]
    fn oblique_clamp_check() {
        let oblique = Oblique::new(270.0, 2.0, 30.0);

        assert!(oblique.angle_rad.eq(&180.0f32.to_radians()));
        assert!(oblique.vertical_position.eq(&1.0));
        assert!(oblique.depth_offset.eq(&30.0));
    }


    #[test]
    fn oblique_uncheck() {
        let oblique = Oblique {
            angle_rad: 1230.0f32.to_radians(),
            vertical_position: 25.0,
            depth_offset: 1579.0,
        };


        assert!(oblique.angle_rad.ne(&180.0f32.to_radians()));
        assert!(oblique.vertical_position.ne(&1.0));
        assert!(oblique.depth_offset.eq(&1579.0));
    }
}
