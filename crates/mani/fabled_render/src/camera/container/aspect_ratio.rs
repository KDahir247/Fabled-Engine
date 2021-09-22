#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AspectRatio {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self {
            horizontal: 16.0,
            vertical: 9.0,
        }
    }
}

impl AspectRatio {
    pub fn new(horizontal: f32, vertical: f32) -> Self {
        let horizontal = horizontal.max(1.0);
        let vertical = vertical.max(1.0);

        Self {
            horizontal,
            vertical,
        }
    }

    pub fn get_aspect(&self) -> f32 {
        self.horizontal / self.vertical
    }
}

#[cfg(test)]
mod aspect_ration_test {
    use crate::camera::AspectRatio;

    #[test]
    fn invalid_aspect_ratio() {
        let aspect_ratio = AspectRatio::new(0.0, 0.0);

        assert!(aspect_ratio.get_aspect().eq(&1.0));

        assert!(aspect_ratio.vertical.eq(&1.0));
        assert!(aspect_ratio.horizontal.eq(&1.0));
    }

    #[test]
    fn invalid_ignore_aspect_ratio() {
        let aspect_ratio = AspectRatio {
            horizontal: 0.0,
            vertical: 0.0,
        };

        assert!(aspect_ratio.horizontal.eq(&0.0));
        assert!(aspect_ratio.vertical.eq(&0.0));

        let nan_operation = aspect_ratio.get_aspect();
        assert!(nan_operation.is_nan());
    }

    #[test]
    fn correct_calculation() {
        let aspect_ratio = AspectRatio::default();
        let aspect = aspect_ratio.get_aspect();
        println!(
            "aspect is {} for {}:{}",
            aspect, aspect_ratio.horizontal, aspect_ratio.vertical
        );
    }
}
