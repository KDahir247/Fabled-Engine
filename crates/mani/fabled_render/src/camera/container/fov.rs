use crate::camera::AspectRatio;

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Fov {
    pub radian: f32,
    pub axis: FovAxis,
}

impl Default for Fov {
    fn default() -> Self {
        Self {
            radian: 60.0f32.to_radians(),
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

            let mut aspect_axis = aspect_w / aspect_h;

            match axis {
                FovAxis::Horizontal => self.axis = FovAxis::Horizontal,
                FovAxis::Vertical => {
                    self.axis = FovAxis::Vertical;
                    aspect_axis = aspect_h / aspect_w;
                }
            }

            let result_fov = 2.0 * ((self.radian / 2.0).tan() * aspect_axis).atan();

            self.radian = result_fov;
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

        let mut fov = Fov::new(horizontal_fv, FovAxis::Horizontal);

        fov.convert_axis(FovAxis::Vertical, AspectRatio::default());

        assert!(fov.radian.to_degrees().eq(&vertical_new.to_degrees()));
    }

    #[test]
    fn conversion_test() {
        let threshold = 0.00001;

        let mut fov = Fov::default();

        let initial_fov = fov.radian;

        fov.convert_axis(FovAxis::Horizontal, AspectRatio::default());

        let fov_horizontal = fov.radian;

        println!("Horizontal fov is {}", fov_horizontal.to_degrees());

        fov.convert_axis(FovAxis::Vertical, AspectRatio::default());

        println!("{} {}", initial_fov.to_degrees(), fov.radian.to_degrees());
        assert!((initial_fov - fov.radian).abs() < threshold);
    }
}
