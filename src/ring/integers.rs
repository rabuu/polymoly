//! The ring of integers `Z`

use super::Ring;

/// The ring of integers `Z`
#[derive(Debug, Clone, Copy)]
pub struct Integers;

impl Ring for Integers {
    type Element = isize;

    fn zero(&self) -> Self::Element {
        0
    }

    fn one(&self) -> Self::Element {
        1
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
