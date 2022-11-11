pub fn flush_underflow_denormal(){
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    unsafe{

        let mut mxcsr = _mm_getcsr();

        mxcsr |= (1 << 15) | (1 << 6);

        mxcsr |= ((1 << 6) - 1) << 7;

        _mm_setcsr(mxcsr);
    }
}