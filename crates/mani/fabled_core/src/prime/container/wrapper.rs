use crate::prime::container::primitive::Primitive;
use serde::{Deserialize, Serialize};

// todo create enum for different size primitive and (Eg. 8, 16, 32, 64, 128, and usize)
//  take the explicit size into account when getting the bytes an length from the generic type as a function.

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Wrapper<T> {
    data: T,
    length: usize,
}

impl<'de, T> Wrapper<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new(value: T) -> Wrapper<T> {
        Self {
            data: value,
            length: std::mem::size_of::<T>(),
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let data = self.data;
        let bytes: Vec<u8> = bytemuck::cast_slice(&[data]).to_vec();
        bytes
    }

    pub fn retrieve_type(bytes: Vec<u8>) -> T {
        let data: &T = bytemuck::from_bytes(bytes.as_slice());
        data.to_owned()
    }

    pub fn retrieve_self(&self) -> T {
        self.data
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length.le(&0)
    }
}

#[cfg(test)]
mod wrapper_test {
    use crate::prime::container::wrapper::Wrapper;

    #[test]
    fn bytes_check() {
        let wrapper_struct = Wrapper::new([10.0, 5.0, 3.0, 1.0]);
        let data = wrapper_struct.get_bytes();
        println!("{:?}", data);

        let a = (10.0, 5.0, 3.0, 1.0);
        let data: Vec<u8> = bytemuck::cast_slice(&[a.0, a.1, a.2, a.3]).to_owned();

        println!("{:?}", data);
    }

    #[test]
    fn undo_bytes_cast() {
        let wrapper_struct = Wrapper::new([10.0, 5.0, 1.0]);
        let data = wrapper_struct.get_bytes();
        let wrapper: [f64; 3] = Wrapper::retrieve_type(data);
        println!("{:?}", wrapper);
    }

    #[test]
    fn retrieve_self() {
        let wrapper = Wrapper::new([10.0, 5.0]);
        let data = wrapper.retrieve_self();
        assert!(data.eq(&[10.0, 5.0]));
    }

    #[test]
    fn copy_test() {
        let wrapper = Wrapper::new(30);
        let wrapper1 = wrapper;
        print!("{:?}", wrapper);
    }
}
