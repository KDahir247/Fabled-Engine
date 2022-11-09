use crate::Vector2;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};


#[derive(Copy, Clone)]
pub struct DualNumber{
    // real, img
    pub value : Vector2
}

impl Default for DualNumber{
    fn default() -> Self {
        DualNumber { value: Vector2::default() }
    }
}

impl DualNumber{
    #[inline(always)]
    pub const fn set(real : f32, img : f32) -> DualNumber{
        DualNumber { value: Vector2::set(real, img) }
    }

    #[inline(always)]
    pub const fn new(dual_num : Vector2) -> DualNumber{
        DualNumber { value: dual_num }
    }

    #[inline(always)]
    pub const fn broadcast(val : f32) -> DualNumber{
        DualNumber { value: Vector2::broadcast(val) }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 2]{
        Vector2::to_primitive(self.value)
    }

    #[inline]
    pub const fn from_primitive(array : [f32; 2]) -> DualNumber{
        DualNumber { value: Vector2::from_primitive(array) }
    }
}

//Component-Wise


// DualNumber-Wise
impl Add for DualNumber{
    type Output = DualNumber;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        DualNumber{ value: self.value + rhs.value}
    }
}

impl AddAssign for DualNumber{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for DualNumber{
    type Output = DualNumber;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        DualNumber{ value: self.value + rhs.value }
    }
}

impl SubAssign for DualNumber{

    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Mul for DualNumber{
    type Output = DualNumber;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        DualNumber::set(
            self.value.x() * rhs.value.x(),
            (self.value.x() * rhs.value.y()) + (self.value.y() * rhs.value.x())
        )
    }
}

impl MulAssign for DualNumber{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = Vector2::set(
            self.value.x() * rhs.value.x(),
            (self.value.x() * rhs.value.y()) + (self.value.y() * rhs.value.x())
        )
    }
}

impl Div for DualNumber{
    type Output = DualNumber;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        DualNumber::set(
            self.value.x() / self.value.x(),
            ((self.value.y() * rhs.value.x()) - (self.value.x() * rhs.value.y())) / (rhs.value.x() * rhs.value.x())
        )
    }
}

impl DivAssign for DualNumber{
    #[inline]
    fn div_assign(&mut self, rhs: Self) {

        self.value = Vector2::set(
            self.value.x() / rhs.value.x(),
           ((self.value.y() * rhs.value.x()) - (self.value.x() * rhs.value.y())) / (rhs.value.x() * rhs.value.x())
        )
    }
}


pub mod dual_number_math{
    use crate::DualNumber;

    #[inline]
    pub fn length_dual_num(dual_number : DualNumber) -> f32{
        (dual_number.value.x() * dual_number.value.x()).sqrt()
    }

    #[inline]
    pub fn length_sqr_dual_num(dual_number : DualNumber) -> f32{
        dual_number.value.x() * dual_number.value.x()
    }
}