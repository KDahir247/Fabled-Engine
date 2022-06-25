pub unsafe trait System<Params, Ret> {
    fn execute(self) -> Box<Self>;
}


macro_rules! impl_execute {
    ($($xs:ident),+ $(,)?) => {
        unsafe impl< $($xs),+, F, R> System< ($($xs,)+), R> for F
        where
            F: FnMut($($xs),+) -> R,
        {
            fn execute(self) -> Box<F> {
                Box::new(self)
            }

        }
    };
}

macro_rules! recursive {
    ($macro:ident, $first:ident) => {
        $macro!($first);
    };
    ($macro:ident, $first:ident, $($rest:ident),* $(,)?) => {
        $macro!($first, $($rest),*);
        recursive!($macro, $($rest),*);
    };
}

recursive!(
    impl_execute,
    P15,
    P14,
    P13,
    P12,
    P11,
    P10,
    P9,
    P8,
    P7,
    P6,
    P5,
    P4,
    P3,
    P2,
    P1,
    P0,
);
