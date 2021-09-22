use serde::{Deserialize, Serialize};

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
        let bytes: Vec<u8> = bytemuck::bytes_of(&self.data).to_vec();
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
        let wrapper_struct = Wrapper::new([10.0, 21.123, 0.5]);
        let wrapped_data = wrapper_struct.get_bytes();

        let a = [10.0, 21.123, 0.5];
        let data: Vec<u8> = bytemuck::cast_slice(&[a]).to_owned();

        assert!(wrapped_data.eq(&data));
        assert!(wrapped_data.len().eq(&data.len()));

        println!("{:?}", wrapped_data);
        println!("{:?}", data);
    }

    #[test]
    fn undo_bytes_cast() {
        let initial_value = [10.0, 5.0, 1.0];

        let wrapper_struct = Wrapper::new(initial_value);
        let data = wrapper_struct.get_bytes();
        let wrapper: [f64; 3] = Wrapper::retrieve_type(data);

        println!("{:?}", wrapper);
    }

    #[test]
    fn retrieve_self() {
        let initial_value = [10.0, 5.0];
        let wrapper = Wrapper::new(initial_value);
        let data = wrapper.retrieve_self();

        assert!(data.eq(&initial_value));
    }
}
