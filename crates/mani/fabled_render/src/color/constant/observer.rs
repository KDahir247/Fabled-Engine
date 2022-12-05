// todo add all illumination white point.

pub mod two_degree_observer {
    use fabled_math::Vector3;

    pub const A_WHITE_POINT: Vector3 = Vector3::set(1.098466069, 1.0, 0.3558228003);
    pub const B_WHITE_POINT: Vector3 = Vector3::set(0.990927448, 1.0, 0.8531327323);
    pub const C_WHITE_POINT: Vector3 = Vector3::set(0.9807059717, 1.0, 1.182249494);
    pub const D50_WHITE_POINT: Vector3 = Vector3::set(0.9642119944, 1.0, 0.8251882845);
    pub const D55_WHITE_POINT: Vector3 = Vector3::set(0.9567970526, 1.0, 0.921480586);
    pub const D65_WHITE_POINT: Vector3 = Vector3::set(0.9504285454, 1.0, 1.088900371);
    pub const D75_WHITE_POINT: Vector3 = Vector3::set(0.9497220899, 1.0, 1.226393521);
}


pub mod ten_degree_observer {
    use fabled_math::Vector3;

    pub const D50_WHITE_POINT: Vector3 = Vector3::set(0.967206275, 1.0, 0.8142801513);
    pub const D55_WHITE_POINT: Vector3 = Vector3::set(0.9579665682, 1.0, 0.909252516);
    pub const D65_WHITE_POINT: Vector3 = Vector3::set(0.9480966767, 1.0, 1.07305136);
    pub const D75_WHITE_POINT: Vector3 = Vector3::set(0.9441713926, 1.0, 1.206427221);
}
