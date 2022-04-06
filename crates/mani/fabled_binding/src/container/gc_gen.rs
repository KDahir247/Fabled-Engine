pub struct Predicate<const COND: bool>;

pub trait True {}

impl True for Predicate<true> {}

#[non_exhaustive]
pub struct GCMultiplier<const LOW: i16, const HIGH: i16> {
    pub multiplier: i16,
}

const fn in_range(v: i16, low: i16, high: i16) -> bool {
    v >= low && v <= high
}

impl<const LOW: i16, const HIGH: i16> GCMultiplier<LOW, HIGH>
where
    Predicate<{ LOW < HIGH }>: True,
{
    pub const fn new<const V: i16>() -> Self
    where
        Predicate<{ in_range(V, LOW, HIGH) }>: True, {
        Self { multiplier: V }
    }
}
