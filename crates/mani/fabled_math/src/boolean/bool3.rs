pub struct Bool3 {
    pub value: std::simd::Mask<i32, 4>,
}


impl Default for Bool3{
    #[inline]
    fn default() -> Self {
        Bool3 { value: std::simd::mask32x4::default() }
    }
}

impl Bool3{

    #[inline(always)]
    pub fn set(x : bool, y : bool, z : bool) -> Bool3{

        todo!()
    }

    #[inline]
    pub fn splat(val : bool) -> Bool3{
        Bool3 {
            value: std::simd::mask32x4::from_array([val, val,val,false])
        }
    }
}
