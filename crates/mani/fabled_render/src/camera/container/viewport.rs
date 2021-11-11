// Will be convert to a Rect when implemented in the math module
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub struct ViewPort {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Default for ViewPort {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }
}

impl ViewPort {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) -> ViewPort {
        let max_depth = max_depth.max(min_depth + 0.1);

        Self {
            x,
            y,
            w: width,
            h: height,
            min_depth,
            max_depth,
        }
    }
}

#[cfg(test)]
mod viewport_tests {
    use crate::camera::ViewPort;

    #[test]
    fn invalid_viewport() {
        let viewport = ViewPort::new(0.0, 0.0, 1.0, 1.0, 1.0, 0.0);
        println!(
            "min depth {} max depth {}",
            viewport.min_depth, viewport.max_depth
        );
        assert!(viewport.max_depth > viewport.min_depth);
    }

    #[test]
    fn invalid_ignore_viewport() {
        let viewport = ViewPort {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
            min_depth: 1.0,
            max_depth: 0.1,
        };

        assert!(viewport.min_depth.gt(&viewport.max_depth));
    }

    #[test]
    fn correct_calculation() {
        let viewport = ViewPort::default();
        print!("{:?}", viewport);
    }
}
