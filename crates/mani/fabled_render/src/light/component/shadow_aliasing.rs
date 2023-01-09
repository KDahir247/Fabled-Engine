
#[derive(Copy, Clone)]
pub struct ParallelSplitVarianceFilter {}


pub trait SupportedKernel {}
pub struct Kernel<const Size: usize>();

// example
impl SupportedKernel for Kernel<4> {}
impl SupportedKernel for Kernel<6> {}
impl SupportedKernel for Kernel<10> {}
impl SupportedKernel for Kernel<14> {}

#[derive(Copy, Clone)]
pub struct PercentageCloserShadow<const FilterSize: usize>
where
    Kernel<{ FilterSize + FilterSize }>: SupportedKernel, {
    pub kernel: [f32; FilterSize + FilterSize],
    pub disk_samples: f32,
}

pub struct PercentageCloserFilter {
    pub filter_size: u32,
}

#[derive(Copy, Clone)]
pub struct VarianceFilter {}
