#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClippingPlane {
    pub far: f32,
    pub near: f32,
}

impl Default for ClippingPlane {
    fn default() -> Self {
        Self {
            far: 1000.,
            near: 0.1,
        }
    }
}

impl ClippingPlane {
    pub fn new(far: f32, near: f32) -> Self {
        let near = near.max(0.01);
        let far = far.max(near + 0.1);

        Self { far, near }
    }
}

#[cfg(test)]
mod clipping_plane_test {
    use crate::camera::ClippingPlane;

    #[test]
    fn invalid_clipping_plane() {
        // near cant be higher then far.
        let clipping_plane = ClippingPlane::new(0.0, 10.0);

        println!("near {} far {}", clipping_plane.near, clipping_plane.far);
        assert!(clipping_plane.far > clipping_plane.near);

        // We cant technically have a zero near for orthographic, but this isn't ok for
        // prospective so we will add the same constraint to orthographic.
        let clipping_plane = ClippingPlane::new(1000.0, 0.0);

        assert!(clipping_plane.near.ne(&0.0));
    }

    #[test]
    fn invalid_ignore_aspect_ratio() {
        let clipping_plane = ClippingPlane {
            far: 0.5,
            near: 10.0,
        };

        assert!(clipping_plane.near.gt(&clipping_plane.far));
    }

    #[test]
    fn correct_calculation() {
        let clipping_plane = ClippingPlane::default();
        println!(
            "near clipping {} far clipping {}",
            clipping_plane.near, clipping_plane.far
        );
    }
}
