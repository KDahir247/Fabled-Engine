#![allow(dead_code)]
use shipyard::{EntityId, Component};

/// ChildOf is the source of truth when it comes to the structure of things in trees.
///
/// .0 is parent EntityId, .1 is Ordered relative to siblings
#[derive(Clone, Debug, PartialEq, Eq, Component)]
#[track(All)]
pub struct ChildOf(pub EntityId, pub Ordered);

impl ChildOf {
    pub fn new(child_of: EntityId, hint: u8) -> Self {
        ChildOf(child_of, Ordered::hinted(hint))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ordered(u32);

impl Ordered {
    /// Create an ordered component with a hint of what it's initial order should be
    pub fn hinted(hint: u8) -> Self {
        const EIGHTH_MAX: u32 = std::u32::MAX / 8;
        Ordered(((hint as u32).pow(3) + (hint as u32) * 4) + EIGHTH_MAX)
    }

    /// Mutate version of "between"
    pub fn move_between(&mut self, min: &Self, max: &Self) {
        self.0 = (min.0 / 2) + (max.0 / 2);
    }

    // 👇 Somewhat thought through ordering logic inspired by fractional indexing

    pub fn between(min: &Self, max: &Self) -> Self {
        Ordered((min.0 / 2) + (max.0 / 2))
    }

    pub fn after(&self) -> Self {
        const HALF_MAX: u32 = std::u32::MAX / 2;
        // average between a and max
        Ordered((self.0 / 2) + HALF_MAX)
    }

    pub fn before(&self) -> Self {
        // I know this is zero, but for posterity let's think about this conceptually as the average between a & min.
        const HALF_MIN: u32 = std::u32::MIN / 2;
        Ordered((self.0 / 2) + HALF_MIN)
    }
}

#[test]
fn before_divides_by_2() {
    let ordered = Ordered(1).before();
    assert_eq!(ordered, Ordered(0));

    let ordered = Ordered(10).before();
    assert_eq!(ordered, Ordered(5));

    let ordered = Ordered(100).before();
    assert_eq!(ordered, Ordered(50));
}

#[test]
fn hinted_cubes_then_adds_self_multuplied_by_4_and_then_adds_an_eigth_of_u32max() {
    const EIGHTH_MAX: u32 = 536870911;
    let ordered = Ordered::hinted(1);
    assert_eq!(ordered, Ordered(1 + 4 + EIGHTH_MAX));

    let ordered = Ordered::hinted(10);
    assert_eq!(ordered, Ordered(1000 + 40 + EIGHTH_MAX));

    let ordered = Ordered::hinted(100);
    assert_eq!(ordered, Ordered(1000000 + 400 + EIGHTH_MAX));
}

#[test]
fn between_finds_the_floored_mid_point_between_two_ordereds() {
    let between = Ordered::between(&Ordered(1), &Ordered(10));
    assert_eq!(between, Ordered(5));

    let between = Ordered::between(&Ordered(1), &Ordered(11));
    assert_eq!(between, Ordered(5));
}

#[test]
fn after_divides_by_2_and_adds_half_of_u32max() {
    const HALF_MAX: u32 = 2147483647;

    let after = Ordered(1).after();
    assert_eq!(after, Ordered(HALF_MAX));

    let after = Ordered(10).after();
    assert_eq!(after, Ordered(5 + HALF_MAX));

    let after = Ordered(100).after();
    assert_eq!(after, Ordered(50 + HALF_MAX));
}

pub struct OrderedRange(u32, u32);

impl OrderedRange {
    pub fn new(from: &Ordered, to: &Ordered) -> Self {
        if from > to {
            panic!("From value must be less than to value")
        }
        OrderedRange(from.0, to.0)
    }

    /// Create a list of evenly spaced Ordered components within this OrderedRange.
    /// The start and end points are not guaranteed to be the same as the `from` and `to`.
    pub fn evenly_spaced_between(&self, length: usize) -> Vec<Ordered> {
        let length = length as u32;
        let distance = self.1 - self.0;
        let step = distance / (length + 1);
        (0..length)
            .map(|i| Ordered((i + 1) * step + self.0))
            .collect()
    }
}

#[test]
#[should_panic(expected = "From value must be less than to value")]
fn ordered_range_must_not_be_greater_than_to() {
    OrderedRange::new(&Ordered(2), &Ordered(1));
}

#[test]
fn evenly_spaced_produces_a_vector_of_ordereds_with_one_distances() {
    let range = OrderedRange::new(&Ordered(1), &Ordered(5)).evenly_spaced_between(3);
    assert_eq!(range, vec![Ordered(2), Ordered(3), Ordered(4)]);
}

#[test]
fn evenly_spaced_between_produces_a_vector_of_ordereds_with_equal_distances() {
    let range = OrderedRange::new(&Ordered(1), &Ordered(10)).evenly_spaced_between(3);
    assert_eq!(range, vec![Ordered(3), Ordered(5), Ordered(7)]);
}
