// If you need more than 10 variables to pass through to the buffer then you should probably create
// a new Buffer and/or normalize your data. You can also extend the BytesArray if absolutely necessary.

//todo Working Progress

type Buffer3<T, U, V> = (T, U, V);

type Buffer4<T, U, V, W> = (T, U, V, W);

type Buffer5<T, U, V, W, X> = (T, U, V, W, X);

// ------------------------ BytesArray ------------------------

#[derive(Copy, Clone)]
pub struct BytesArray<Q>(pub Q);

impl<Q> BytesArray<Q>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let buf = vec![self.0];
        bytemuck::cast_slice(&buf).to_owned()
    }

    pub fn retrieve_type<T: bytemuck::Pod>(&self, bytes: Vec<u8>) -> T {
        let data: &T = bytemuck::from_bytes(&bytes);

        data.to_owned()
    }
}

// ------------------------ Bytes2Array ------------------------

#[derive(Copy, Clone)]
pub struct Bytes2Array<Q, R>(Q, R);

impl<Q, R> Bytes2Array<Q, R>
where
    Q: bytemuck::Pod + bytemuck::Zeroable,
    R: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn retrieve_bytes(&self) -> Vec<u8> {
        let first = vec![self.0];
        let second = vec![self.1];

        let mut prepend: Vec<u8> = bytemuck::cast_slice(&first).to_vec();

        {
            let mut append: Vec<u8> = bytemuck::cast_slice(&second).to_vec();
            prepend.append(&mut append);
        }

        prepend
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s) = self.0;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s, t) = self.0;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s, t, u) = self.0;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[u]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s) = self.0;
        let (t, u, v) = self.1;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[u]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[v]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s, t) = self.0;
        let (u, v, w) = self.1;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[u]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[v]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[w]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s, t) = self.0;
        let (u, v, w, x) = self.1;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[u]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[v]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[w]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[x]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s, t, u) = self.0;
        let (v, w, x, y) = self.1;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[u]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[v]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[w]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[x]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[y]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
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
        let (q, r, s, t, u) = self.0;
        let (v, w, x, y, z) = self.1;

        let mut buff_container: Vec<u8> = bytemuck::cast_slice(&[q]).to_vec();

        {
            let mut append = bytemuck::cast_slice(&[r]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[s]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[t]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[u]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[v]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[w]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[x]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[y]).to_vec();
            buff_container.append(&mut append);

            let mut append = bytemuck::cast_slice(&[z]).to_vec();
            buff_container.append(&mut append);
        }

        buff_container
    }

    pub fn retrieve_type<A: bytemuck::Pod>(&self, bytes: Vec<u8>) -> A {
        let data: &A = bytemuck::from_bytes(&bytes);
        data.to_owned()
    }
}

#[cfg(test)]
mod test {
    use crate::util::container::bytes::{Bytes2Array, Bytes3Array, BytesArray};

    #[test]
    fn byte_test() {
        #[derive(Default, Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
        #[repr(C)]
        struct DummyStruct {
            arg0: i32,
        }

        let dummy = DummyStruct::default();

        let dummy_bytes: Vec<u8> = bytemuck::cast_slice(&[dummy]).to_owned();

        let byte_array = BytesArray { 0: 50 };
        let buff_data = byte_array.retrieve_bytes();

        assert!(dummy_bytes.len().eq(&buff_data.len()));

        //dummy has arg0 at 0, since it calling default.
        assert!(dummy_bytes.ne(&buff_data));

        let new_dummy: DummyStruct = byte_array.retrieve_type(buff_data);

        println!(
            "Changed DummyStruct {:?}\nwith private member to {:?}",
            dummy, new_dummy
        );
    }

    #[test]
    fn byte2_test() {
        #[derive(Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
        #[repr(C)]
        struct Dummy2Struct {
            pub arg0: [f32; 3],
            pub arg1: f32,
            pub arg2: [f32; 2],
        }

        #[derive(Debug, bytemuck::Zeroable, bytemuck::Pod, Copy, Clone)]
        #[repr(C)]
        struct Dummy2Struct1 {
            pub arg0: [f32; 3],
            pub arg1: f32,
            pub arg2: [f32; 2],
            pub arg3: [i32; 2],
        }

        let dummy_missing_arg3 = Dummy2Struct {
            arg0: [3.0, 1.0, 10.0],
            arg1: 20.0,
            arg2: [1.5, 2.5],
        };

        let dummy = Dummy2Struct1 {
            arg0: [3.0, 1.0, 10.0],
            arg1: 20.0,
            arg2: [1.5, 2.5],
            arg3: [32, 100],
        };

        let byte_array: Bytes2Array<Dummy2Struct, [i32; 2]> = Bytes2Array {
            0: dummy_missing_arg3,
            1: [32, 100],
        };

        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[dummy]).to_vec();
        let byte_buffer02: Vec<u8> = byte_array.retrieve_bytes();

        assert_eq!(byte_buffer01.len(), byte_buffer02.len());

        assert!(byte_buffer01.eq(&byte_buffer02));

        let struct_data: Dummy2Struct1 = byte_array.retrieve_type::<Dummy2Struct1>(byte_buffer02);

        println!(
            "Converted Dummy2Struct {:?}\n to Dummy2Struct1 {:?}",
            dummy_missing_arg3, struct_data
        );
    }

    #[test]
    fn bytes3_test() {
        #[derive(Debug, bytemuck::Pod, bytemuck::Zeroable, Copy, Clone)]
        #[repr(C)]
        struct DummyStruct {
            pub arg0: [f32; 5],
            pub arg1: i32,
            pub arg2: f64,
        }

        let dummy = DummyStruct {
            arg0: [2.0, 1.0, 0.3, 0.01, 0.234],
            arg1: 50,
            arg2: 1.234567,
        };

        let byte_array = Bytes3Array {
            0: (
                [2.0f32, 1.0f32, 0.3f32, 0.01f32, 0.234f32],
                50i32,
                1.234567f64,
            ),
        };

        let byte_buffer = byte_array.retrieve_bytes().to_vec();
        let byte_buffer01: Vec<u8> = bytemuck::cast_slice(&[dummy]).to_vec();

        assert!(byte_buffer01.len().eq(&byte_buffer.len()));
        assert!(byte_buffer01.eq(&byte_buffer));

        let data = byte_array.retrieve_type::<DummyStruct>(byte_buffer);

        println!("{:?}\n{:?}", dummy, data);
    }

    #[test]
    fn bytes4_test() {}

    #[test]
    fn bytes5_test() {}

    #[test]
    fn bytes6_test() {}

    #[test]
    fn bytes7_test() {}

    #[test]
    fn bytes8_test() {}

    #[test]
    fn bytes9_test() {}

    #[test]
    fn bytes10_test() {}
}
