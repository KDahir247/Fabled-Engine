use crate::camera::AspectRatio;

//todo future implementation
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FovScalingAlgorithm {
    HorizontalPlus,
    Anamorphic,
    //PixelBased, Not using since the engine is targeted to 3d
    VerticalMinus,
    Stretch,
}

impl Default for FovScalingAlgorithm {
    fn default() -> Self {
        Self::HorizontalPlus
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FovAxis {
    Horizontal = 0,
    Vertical = 1,
}

impl Default for FovAxis {
    fn default() -> Self {
        Self::Vertical
    }
}

pub struct Fov {
    pub radian: f32,
    pub axis: FovAxis,
}

impl Default for Fov {
    fn default() -> Self {
        Self {
            radian: std::f32::consts::FRAC_PI_3,
            axis: Default::default(),
        }
    }
}

impl Fov {
    pub fn new(radian: f32, axis: FovAxis) -> Self {
        Self { radian, axis }
    }

    pub fn convert_axis(&mut self, axis: FovAxis, aspect_ratio: AspectRatio) {
        if axis != self.axis {
            let aspect_w = aspect_ratio.horizontal;
            let aspect_h = aspect_ratio.vertical;

            match axis {
                FovAxis::Horizontal => {
                    self.axis = FovAxis::Horizontal;

                    let horizontal_fov =
                        2.0 * ((self.radian / 2.0).tan() * aspect_w / aspect_h).atan();

                    self.radian = horizontal_fov;
                }
                FovAxis::Vertical => {
                    self.axis = FovAxis::Vertical;

                    let vertical_fov =
                        2.0 * ((self.radian / 2.0).tan() * aspect_h / aspect_w).atan();

                    self.radian = vertical_fov;
                }
            }
        }
    }
}

#[cfg(test)]
mod fov_test {
    use crate::camera::{AspectRatio, Fov, FovAxis};

    #[test]
    fn vertical_to_horizontal() {
        let horizontal = 16.0;
        let vertical = 9.0;

        let horizontal_fv = 90.0f32.to_radians();

        let vertical_new = 2.0 * ((horizontal_fv / 2.0).tan() * vertical / horizontal).atan();
        let horizontal_new = 2.0 * ((vertical_new / 2.0).tan() * horizontal / vertical).atan();

        println!(
            "vertical fov is {} for horizontal fov {}",
            vertical_new.to_degrees(),
            horizontal_fv.to_degrees()
        );
        assert!(horizontal_new.eq(&horizontal_fv));
    }

    #[test]
    fn conversion_test() {
        let mut fov = Fov::default();

        let initial_fov = fov.radian;

        fov.convert_axis(FovAxis::Horizontal, AspectRatio::default());

        let fov_horizontal = fov.radian;

        println!("Horizontal fov is {}", fov_horizontal.to_degrees());

        fov.convert_axis(FovAxis::Vertical, AspectRatio::default());

        assert!(initial_fov.to_degrees().eq(&fov.radian.to_degrees()));
    }
}
