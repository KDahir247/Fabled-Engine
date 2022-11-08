use crate::Vector2;

#[derive(Copy, Clone)]
pub struct DualNumber{
    // real, img
    value : Vector2
}

impl Default for DualNumber{
    fn default() -> Self {
        DualNumber { value: Vector2::default() }
    }
}

impl DualNumber{

}