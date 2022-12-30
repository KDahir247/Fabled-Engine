use crate::camera::fov_conversion::focal_length_to_directional_fov;
use crate::camera::{AnamorphicDescriptor, AspectRatio, FovScalingAlgorithm};
use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};

// Optional add component if camera is Perspective.

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FovAxis {
    Horizontal = 0,
    Vertical = 1,
}

impl Default for FovAxis {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Display for FovAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_repr = match self {
            FovAxis::Horizontal => "Horizontal",
            FovAxis::Vertical => "Vertical",
        };

        f.write_str(string_repr)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Fov {
    pub radian: f32,
    pub axis: FovAxis,
}

impl Default for Fov {
    fn default() -> Fov {
        Fov {
            radian: 60.0f32.to_radians(),
            axis: Default::default(),
        }
    }
}

impl Fov {
    pub const fn new(fov_radian: f32, fov_axis: FovAxis) -> Fov {
        Fov {
            radian: fov_radian,
            axis: fov_axis,
        }
    }

    pub fn new_with_algorithm(
        horizontal_radian: f32,
        fov_algorithm: FovScalingAlgorithm,
    ) -> (Fov, AspectRatio) {
        match fov_algorithm {
            FovScalingAlgorithm::HorizontalPlus {
                target_aspect,
                current_aspect,
            } => {
                let horizontal_plus_fov =
                    Fov::horizontal_plus_fov(horizontal_radian, target_aspect, current_aspect);

                (horizontal_plus_fov, current_aspect)
            }
            FovScalingAlgorithm::Anamorphic {
                len_type,
                frame_aperture,
                anamorphic_descriptor,
            } => {
                let (focal_length_horizontal, aspect_w) =
                    Fov::anamorphic_fov(anamorphic_descriptor);

                let (horizontal_fov, _horizontal_magnification) = focal_length_to_directional_fov(
                    focal_length_horizontal,
                    frame_aperture,
                    Some(anamorphic_descriptor.focus_distance),
                    anamorphic_descriptor.crop_factor,
                    len_type,
                );

                let res_fov = Fov {
                    radian: horizontal_fov,
                    axis: FovAxis::Horizontal,
                };

                let res_aspect = AspectRatio::new(aspect_w, 1.0);

                (res_fov, res_aspect)
            }
        }
    }

    fn anamorphic_fov(anamorphic_descriptor: AnamorphicDescriptor) -> (f32, f32) {
        let clamped_focus_distance = anamorphic_descriptor.focus_distance.clamp(0.0, 100.0);

        let crop_factor = anamorphic_descriptor.crop_factor.unwrap_or(1.0);
        let focal_reducer = anamorphic_descriptor.focal_reducer.unwrap_or(1.0);

        let rev_flip_focus_distance = (clamped_focus_distance - 100.0) * 0.1;
        let rev_aspect_ratio_rcp =
            (anamorphic_descriptor.sensor_aspect_ratio.get_aspect() / 1.78).recip();


        let square_fit_wide = (1.0 - anamorphic_descriptor.single_focus_solution.0)
            * rev_flip_focus_distance.powf(2.0);

        let adapter_rcp = anamorphic_descriptor.anamorphic_adapter.recip();

        let mut horizontal_focal_length = ((crop_factor * focal_reducer)
            * (anamorphic_descriptor.focal_length * adapter_rcp))
            * rev_aspect_ratio_rcp;

        horizontal_focal_length *= 1.0 - square_fit_wide;

        let aspect_ratio = anamorphic_descriptor.sensor_aspect_ratio.get_aspect()
            * anamorphic_descriptor.anamorphic_adapter;
        (horizontal_focal_length, aspect_ratio)
    }

    fn horizontal_plus_fov(
        target_fov_h_rad: f32,
        target_aspect: AspectRatio,
        current_aspect: AspectRatio,
    ) -> Fov {
        let mut fov_res = Fov {
            radian: target_fov_h_rad,
            axis: FovAxis::Horizontal,
        };
        fov_res.convert_axis(FovAxis::Vertical, target_aspect);

        fov_res.convert_axis(FovAxis::Horizontal, current_aspect);

        fov_res
    }

    pub fn convert_axis(&mut self, axis: FovAxis, aspect_ratio: AspectRatio) {
        if axis != self.axis {
            let aspect_w = aspect_ratio.horizontal;
            let aspect_h = aspect_ratio.vertical;

            let vertical = aspect_h / aspect_w;
            let horizontal = aspect_w / aspect_h;
            let mut axis_instructions = horizontal;

            match axis {
                FovAxis::Horizontal => {
                    self.axis = FovAxis::Horizontal;
                }
                FovAxis::Vertical => {
                    self.axis = FovAxis::Vertical;
                    axis_instructions = vertical;
                }
            }

            self.radian = 2.0 * ((self.radian / 2.0).tan() * axis_instructions).atan();
        }
    }
}

impl Component for Fov {
    type Tracking = Modification;
}

impl Display for Fov {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fov(angle radian : {}, axis : {})",
            self.radian, self.axis
        )
    }
}

#[cfg(test)]
mod fov_test {
    use crate::camera::{AspectRatio, Fov, FovAxis};

    #[test]
    fn diagonal_test() {
        let horizontal_fv = 17.5f32.to_radians();

        let mut fov = Fov::new(horizontal_fv, FovAxis::Horizontal);

        fov.convert_axis(FovAxis::Vertical, AspectRatio::default());
        println!("{}", fov.radian.to_degrees());
    }

    #[test]
    fn vertical_to_horizontal() {
        let horizontal = 16.0;
        let vertical = 9.0;

        let horizontal_fv = 17.5f32.to_radians();

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
        fov.convert_axis(FovAxis::Horizontal, AspectRatio::default());

        assert!(fov.radian.to_degrees().eq(&horizontal_fv.to_degrees()));
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
