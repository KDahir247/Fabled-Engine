#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AxisDirection {
    Origin = 0b00000011, // 00000011
    X = 0b00000010,      // 00000010
    Y = 0b00000001,      // 00000001
}

#[cfg(test)]
mod byte_test {
    use crate::container::AxisDirection;

    #[test]
    fn direction() {
        let origin = AxisDirection::Origin;
        let origin_i32 = origin as i32;

        let c0_r0 = -(origin_i32 & 1).signum();
        let c1_r1 = -(origin_i32 & 2).signum();

        // Reflection about the origin
        //[-1, 0, 0]
        //[0, -1, 0]
        //[0, 0, 1.]

        assert!(c0_r0.eq(&-1));
        assert!(c1_r1.eq(&-1));

        println!(" column 0 row 0 : {}, column 1 row 1 : {}", c0_r0, c1_r1);

        let x_axis = AxisDirection::X;
        let x_axis_i32 = x_axis as i32;

        let c0_r0 = -(x_axis_i32 & 1).signum();
        let c1_r1 = -(x_axis_i32 & 2).signum();

        // Reflection about the x-axis
        // [1,  0, 0]
        // [0, -1, 0]
        // [0,  0, 1]

        // currently 1 is mapped to 0 will change later
        assert!(c0_r0.eq(&0));
        assert!(c1_r1.eq(&-1));

        println!(" column 0 row 0 : {}, column 1 row 1 : {}", c0_r0, c1_r1);

        let y_axis = AxisDirection::Y;
        let y_axis_i32 = y_axis as i32;

        let c0_r0 = -(y_axis_i32 & 1).signum();
        let c1_r1 = -(y_axis_i32 & 2).signum();

        // Reflection about the y-axis
        // [-1, 0, 0]
        // [0,  1, 0]
        // [0,  0, 1]

        assert!(c0_r0.eq(&-1));
        assert!(c1_r1.eq(&0));

        println!(" column 0 row 0 : {}, column 1 row 1 : {}", c0_r0, c1_r1);
    }
}
