//! The ring of real numbers `R`

use super::{Field, Ring};

/// The ring of real numbers `R`
#[derive(Debug)]
pub struct R;

impl Ring for R {
    type Element = f64;

    fn zero() -> Self::Element {
        0.0
    }

    fn one() -> Self::Element {
        1.0
    }

    fn add(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        lhs + rhs
    }

    fn neg(elem: Self::Element) -> Self::Element {
        -elem
    }

    fn mul(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        lhs * rhs
    }
}

impl Field for R {
    fn inv(elem: Self::Element) -> Option<Self::Element> {
        (elem != 0.0).then_some(1.0 / elem)
    }
}
