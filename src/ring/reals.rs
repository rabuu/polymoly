use super::{Field, Ring};

/// The ring `R` of real numbers
#[derive(Debug, Clone, Copy)]
pub struct Reals;

impl Ring for Reals {
    type Element = f64;

    fn zero(&self) -> Self::Element {
        0.0
    }

    fn one(&self) -> Self::Element {
        1.0
    }

    fn add(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        lhs + rhs
    }

    fn neg(&self, elem: Self::Element) -> Self::Element {
        -elem
    }

    fn mul(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        lhs * rhs
    }
}

impl Field for Reals {
    fn inv(&self, elem: Self::Element) -> Option<Self::Element> {
        (elem != 0.0).then(|| 1.0 / elem)
    }
}
