use super::Ring;

#[derive(Debug)]
pub struct Z;

impl Ring for Z {
    type Element = isize;

    fn zero() -> Self::Element {
        0
    }

    fn one() -> Self::Element {
        1
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
