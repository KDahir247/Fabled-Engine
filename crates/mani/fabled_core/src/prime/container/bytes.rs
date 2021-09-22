// Generic are constrained to Plain Old Types that can Zeroed and struct that
// contain only POD that can Zeroed.

use crate::prime::container::wrapper::Wrapper;

type WR<T> = Wrapper<T>;

type Buffer2<T, U> = (WR<T>, WR<U>);

type Buffer3<T, U, V> = (WR<T>, WR<U>, WR<V>);

type Buffer4<T, U, V, W> = (WR<T>, WR<U>, WR<V>, WR<W>);

type Buffer5<T, U, V, W, X> = (WR<T>, WR<U>, WR<V>, WR<W>, WR<X>);

// ------------------------ BytesArray ------------------------

#[derive(Copy, Clone)]
pub struct BytesArray<Q>(WR<Q>);

impl<Q> BytesArray<Q>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let data_wrapper = self.0;
        data_wrapper.get_bytes()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let buffer = self.0.get_bytes();
        let result: &A = bytemuck::from_bytes(&buffer);
        result.to_owned()
    }

    pub fn retrieve_self(&self) -> Q {
        self.0.retrieve_self()
    }
}

// ------------------------ Bytes2Array ------------------------

#[derive(Copy, Clone)]
pub struct Bytes2Array<Q, R>((WR<Q>, WR<R>));

impl<Q, R> Bytes2Array<Q, R>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let (wr_x, wr_y) = self.0;
        let buffer_length = wr_x.len() + wr_y.len();

        let mut byte_buffer = vec![0_u8; buffer_length];

        {
            byte_buffer[..wr_x.len()].copy_from_slice(&wr_x.get_bytes());
            byte_buffer[wr_x.len()..].copy_from_slice(&wr_y.get_bytes());
        }
        byte_buffer
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    pub fn retrieve_self(&self) -> Buffer2<Q, R> {
        self.0
    }
}

// ------------------------ Bytes3Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes3Array<Q, R, S>(Buffer3<Q, R, S>);

impl<Q, R, S> Bytes3Array<Q, R, S>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let (wr_x, wr_y, wr_z) = self.0;
        let buffer_length = wr_x.len() + wr_y.len() + wr_z.len();
        let mut byte_buffer = vec![0_u8; buffer_length];

        {
            let offset = wr_x.len();
            let stride = offset + wr_y.len();

            byte_buffer[..offset].copy_from_slice(&wr_x.get_bytes());
            byte_buffer[offset..stride].copy_from_slice(&wr_y.get_bytes());
            byte_buffer[stride..].copy_from_slice(&wr_z.get_bytes());
        }
        byte_buffer
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    pub fn retrieve_self(&self) -> Buffer3<Q, R, S> {
        self.0
    }
}

// ------------------------ Bytes4Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes4Array<Q, R, S, T>(Buffer4<Q, R, S, T>);

impl<Q, R, S, T> Bytes4Array<Q, R, S, T>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let (wr_w, wr_x, wr_y, wr_z) = self.0;
        let buffer_length = wr_w.len() + wr_x.len() + wr_y.len() + wr_z.len();

        let mut byte_buffer = vec![0_u8; buffer_length];

        {
            let offset = wr_w.len();
            let stride = offset + wr_x.len();
            let stride1 = stride + wr_y.len();

            byte_buffer[..offset].copy_from_slice(&wr_w.get_bytes());
            byte_buffer[offset..stride].copy_from_slice(&wr_x.get_bytes());
            byte_buffer[stride..stride1].copy_from_slice(&wr_y.get_bytes());
            byte_buffer[stride1..].copy_from_slice(&wr_z.get_bytes());
        }

        byte_buffer
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    pub fn retrieve_self(&self) -> Buffer4<Q, R, S, T> {
        self.0
    }
}

// ------------------------ Bytes5Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes5Array<Q, R, S, T, U>(Buffer5<Q, R, S, T, U>);

impl<Q, R, S, T, U> Bytes5Array<Q, R, S, T, U>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let (wr_v, wr_w, wr_x, wr_y, wr_z) = self.0;
        let buffer_length = wr_v.len() + wr_w.len() + wr_x.len() + wr_y.len() + wr_z.len();

        let mut byte_buffer = vec![0_u8; buffer_length];

        {
            let offset = wr_v.len();
            let stride = offset + wr_w.len();
            let stride1 = stride + wr_x.len();
            let end = stride1 + wr_y.len();

            byte_buffer[..offset].copy_from_slice(&wr_v.get_bytes());
            byte_buffer[offset..stride].copy_from_slice(&wr_w.get_bytes());
            byte_buffer[stride..stride1].copy_from_slice(&wr_x.get_bytes());
            byte_buffer[stride1..end].copy_from_slice(&wr_y.get_bytes());
            byte_buffer[end..].copy_from_slice(&wr_z.get_bytes());
        }

        byte_buffer
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    pub fn retrieve_self(&self) -> Buffer5<Q, R, S, T, U> {
        self.0
    }
}

// ------------------------ Bytes6Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes6Array<Q, R, S, T, U, V>(Buffer3<Q, R, S>, Buffer3<T, U, V>);

impl<Q, R, S, T, U, V> Bytes6Array<Q, R, S, T, U, V>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let byte_buffer3_prepend = Bytes3Array { 0: self.0 };

        {
            let byte_buffer3_append = Bytes3Array { 0: self.1 };

            byte_buffer3_prepend
                .retrieve_bytes()
                .append(&mut byte_buffer3_append.retrieve_bytes());
        }

        byte_buffer3_prepend.retrieve_bytes()
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    pub fn retrieve_self(&self) -> (Buffer3<Q, R, S>, Buffer3<T, U, V>) {
        (self.0, self.1)
    }
}

// ------------------------ Bytes7Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes7Array<Q, R, S, T, U, V, W>(Buffer4<Q, R, S, T>, Buffer3<U, V, W>);

impl<Q, R, S, T, U, V, W> Bytes7Array<Q, R, S, T, U, V, W>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let byte_buffer4_prepend = Bytes4Array { 0: self.0 };

        {
            let byte_buffer3_append = Bytes3Array { 0: self.1 };

            byte_buffer4_prepend
                .retrieve_bytes()
                .append(&mut byte_buffer3_append.retrieve_bytes());
        }

        byte_buffer4_prepend.retrieve_bytes()
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    #[allow(clippy::type_complexity)]
    pub fn retrieve_self(&self) -> (Buffer4<Q, R, S, T>, Buffer3<U, V, W>) {
        (self.0, self.1)
    }
}

// ------------------------ Bytes8Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes8Array<Q, R, S, T, U, V, W, X>(Buffer4<Q, R, S, T>, Buffer4<U, V, W, X>);

impl<Q, R, S, T, U, V, W, X> Bytes8Array<Q, R, S, T, U, V, W, X>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
    X: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let byte_buffer4_prepend = Bytes4Array { 0: self.0 };

        {
            let byte_buffer4_append = Bytes4Array { 0: self.1 };

            byte_buffer4_prepend
                .retrieve_bytes()
                .append(&mut byte_buffer4_append.retrieve_bytes());
        }

        byte_buffer4_prepend.retrieve_bytes()
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    #[allow(clippy::type_complexity)]
    pub fn retrieve_self(&self) -> (Buffer4<Q, R, S, T>, Buffer4<U, V, W, X>) {
        (self.0, self.1)
    }
}
// ------------------------ Bytes9Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes9Array<Q, R, S, T, U, V, W, X, Y>(Buffer5<Q, R, S, T, U>, Buffer4<V, W, X, Y>);

impl<Q, R, S, T, U, V, W, X, Y> Bytes9Array<Q, R, S, T, U, V, W, X, Y>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
    X: bytemuck::Pod + bytemuck::Zeroable,
    Y: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let byte_buffer5_prepend = Bytes5Array { 0: self.0 };

        {
            let byte_buffer4_append = Bytes4Array { 0: self.1 };

            byte_buffer5_prepend
                .retrieve_bytes()
                .append(&mut byte_buffer4_append.retrieve_bytes());
        }

        byte_buffer5_prepend.retrieve_bytes()
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    #[allow(clippy::type_complexity)]
    pub fn retrieve_self(&self) -> (Buffer5<Q, R, S, T, U>, Buffer4<V, W, X, Y>) {
        (self.0, self.1)
    }
}
// ------------------------ Bytes10Array ------------------------
#[derive(Copy, Clone)]
pub struct Bytes10Array<Q, R, S, T, U, V, W, X, Y, Z>(
    Buffer5<Q, R, S, T, U>,
    Buffer5<V, W, X, Y, Z>,
);

impl<Q, R, S, T, U, V, W, X, Y, Z> Bytes10Array<Q, R, S, T, U, V, W, X, Y, Z>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
    S: bytemuck::Pod + bytemuck::Zeroable,
    T: bytemuck::Pod + bytemuck::Zeroable,
    U: bytemuck::Pod + bytemuck::Zeroable,
    V: bytemuck::Pod + bytemuck::Zeroable,
    W: bytemuck::Pod + bytemuck::Zeroable,
    X: bytemuck::Pod + bytemuck::Zeroable,
    Y: bytemuck::Pod + bytemuck::Zeroable,
    Z: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let byte_buffer5_prepend = Bytes5Array { 0: self.0 };

        {
            let byte_buffer5_append = Bytes5Array { 0: self.1 };

            byte_buffer5_prepend
                .retrieve_bytes()
                .append(&mut byte_buffer5_append.retrieve_bytes());
        }

        byte_buffer5_prepend.retrieve_bytes()
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }

    pub fn cast_to<A: bytemuck::Pod>(&self) -> A {
        let byte_buffer = self.retrieve_bytes();
        let result: &A = bytemuck::from_bytes(&byte_buffer);

        result.to_owned()
    }

    #[allow(clippy::type_complexity)]
    pub fn retrieve_self(&self) -> (Buffer5<Q, R, S, T, U>, Buffer5<V, W, X, Y, Z>) {
        (self.0, self.1)
    }
}
// ------------------------ TEST ------------------------

#[cfg(test)]
mod test {
    use crate::prime::container::bytes::{
        Bytes2Array, Bytes3Array, Bytes4Array, Bytes5Array, BytesArray, WR,
    };

    #[test]
    fn byte_test() {
        #[derive(Default, Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
        #[repr(C)]
        struct MockStruct {
            arg0: i32,
        }

        let dummy = MockStruct::default();
        let dummy_bytes: Vec<u8> = bytemuck::cast_slice(&[dummy]).to_owned();

        let byte_array = BytesArray { 0: WR::new(50) };
        let buff_data = byte_array.retrieve_bytes();

        assert!(dummy_bytes.len().eq(&buff_data.len()));
        assert!(dummy_bytes.ne(&buff_data));

        let new_dummy = byte_array.retrieve_self();
        let mimic_dummy = byte_array.cast_to::<MockStruct>();

        println!(
            "Changed DummyStruct {:?}\nwith private member to {:?}\nThe mimic dummy is now {:?}",
            dummy, new_dummy, mimic_dummy
        );
    }

    #[test]
    fn byte2_test() {
        #[derive(Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
        #[repr(C)]
        struct MockStruct {
            pub arg0: [f32; 3],
            pub arg1: f32,
            pub arg2: [f32; 2],
        }

        #[derive(Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
        #[repr(C)]
        struct MockStruct1 {
            pub arg0: [f32; 3],
            pub arg1: f32,
            pub arg2: [f32; 2],
            pub arg3: [i32; 2],
        }

        let mock_missing_arg3 = MockStruct {
            arg0: [3.0, 1.0, 10.0],
            arg1: 20.0,
            arg2: [1.5, 2.5],
        };

        let mock = MockStruct1 {
            arg0: [3.0, 1.0, 10.0],
            arg1: 20.0,
            arg2: [1.5, 2.5],
            arg3: [32, 100],
        };

        let byte_array = Bytes2Array {
            0: (WR::new(mock_missing_arg3), WR::new([32, 100])),
        };

        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[mock]).to_vec();
        let byte_buffer02: Vec<u8> = byte_array.retrieve_bytes();

        assert_eq!(byte_buffer01.len(), byte_buffer02.len());

        assert!(byte_buffer01.eq(&byte_buffer02));

        let struct_data = byte_array.cast_to::<MockStruct1>();

        println!(
            "Converted Dummy2Struct {:?}\n to Dummy2Struct1 {:?}",
            mock_missing_arg3, struct_data
        );
    }

    #[test]
    fn bytes3_test() {
        #[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
        #[repr(C)]
        struct MockStruct {
            pub arg0: [f32; 5],
            pub arg1: i32,
            pub arg2: f64,
        }

        let mock = MockStruct {
            arg0: [2.0, 1.0, 0.3, 0.01, 0.234],
            arg1: 50,
            arg2: 1.234567,
        };

        let byte_array = Bytes3Array {
            0: (
                WR::new([2.0f32, 1.0f32, 0.3f32, 0.01f32, 0.234f32]),
                WR::new(50i32),
                WR::new(1.234567f64),
            ),
        };

        let byte_buffer = byte_array.retrieve_bytes().to_vec();
        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[mock]).to_vec();

        assert!(byte_buffer01.len().eq(&byte_buffer.len()));
        assert!(byte_buffer01.eq(&byte_buffer));

        let data = byte_array.cast_to::<MockStruct>();

        println!("{:?}\n{:?}", mock, data);
    }

    #[test]
    fn bytes4_test() {
        #[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
        #[repr(C)]
        struct MockStruct {
            pub arg0: [f32; 5],
            pub arg1: i32,
            pub arg2: f64,
            pub arg3: i64,
        }

        let mock = MockStruct {
            arg0: [2.0, 1.0, 0.3, 0.01, 0.234],
            arg1: 50,
            arg2: 1.234567,
            arg3: 105,
        };

        let byte_array = Bytes4Array {
            0: (
                WR::new([2.0f32, 1.0f32, 0.3f32, 0.01f32, 0.234f32]),
                WR::new(50i32),
                WR::new(1.234567f64),
                WR::new(105i64),
            ),
        };

        let byte_buffer = byte_array.retrieve_bytes().to_vec();
        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[mock]).to_vec();

        assert!(byte_buffer01.len().eq(&byte_buffer.len()));
        assert!(byte_buffer01.eq(&byte_buffer));

        let data = byte_array.cast_to::<MockStruct>();

        println!("{:?}\n{:?}", mock, data);
    }

    #[test]
    fn bytes5_test() {
        #[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
        #[repr(C)]
        struct MockStruct {
            pub arg0: [f32; 5],
            pub arg1: i32,
            pub arg2: f64,
            pub arg3: i64,
            arg4: usize,
        }

        let mock = MockStruct {
            arg0: [2.0, 1.0, 0.3, 0.01, 0.234],
            arg1: 50,
            arg2: 1.234567,
            arg3: 105,
            arg4: 10000,
        };

        let byte_array = Bytes5Array {
            0: (
                WR::new([2.0f32, 1.0f32, 0.3f32, 0.01f32, 0.234f32]),
                WR::new(50i32),
                WR::new(1.234567f64),
                WR::new(105i64),
                WR::new(10000usize),
            ),
        };

        let byte_buffer = byte_array.retrieve_bytes().to_vec();
        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[mock]).to_vec();

        assert!(byte_buffer01.len().eq(&byte_buffer.len()));
        assert!(byte_buffer01.eq(&byte_buffer));

        let data = byte_array.cast_to::<MockStruct>();

        println!("{:?}\n{:?}", mock, data);
    }
}
