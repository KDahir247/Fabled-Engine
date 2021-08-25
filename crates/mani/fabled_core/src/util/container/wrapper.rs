use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Wrapper<T> {
    data: T,
}

impl<'de, T> Wrapper<T>
where
    T: Serialize + Deserialize<'de> + bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new(value: T) -> Wrapper<T> {
        Self { data: value }
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
}

#[cfg(test)]
mod wrapper_test {
    use crate::util::container::wrapper::Wrapper;

    #[test]
    fn bytes_check() {
        let wrapper_struct = Wrapper::new([10.0, 5.0, 3.0, 1.0]);
        let data = wrapper_struct.get_bytes();
        println!("{:?}", data);

        let data: &[u8] = bytemuck::cast_slice(&[10.0, 5.0, 3.0, 1.0]);
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
    fn retrieve_self() {}
}
