use crate::Poly;

use super::{PolyRing, ZMod, R, Z};

/// A commutative ring
pub trait Ring {
    type Element;

    fn zero() -> Self::Element;
    fn one() -> Self::Element;

    fn add(lhs: Self::Element, rhs: Self::Element) -> Self::Element;
    fn neg(elem: Self::Element) -> Self::Element;
    fn mul(lhs: Self::Element, rhs: Self::Element) -> Self::Element;

    fn sub(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        Self::add(lhs, Self::neg(rhs))
    }

    fn id(elem: Self::Element) -> Self::Element {
        elem
    }
}

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

impl<const N: usize> Ring for ZMod<N> {
    type Element = isize;

    fn zero() -> Self::Element {
        0
    }

    fn one() -> Self::Element {
        1
    }

    fn add(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        Self::id(lhs + rhs)
    }

    fn neg(elem: Self::Element) -> Self::Element {
        Self::id(-elem)
    }

    fn mul(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        Self::id(lhs * rhs)
    }

    fn id(elem: Self::Element) -> Self::Element {
        elem.rem_euclid(N as isize)
    }
}

impl<R> Ring for PolyRing<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    type Element = Poly<R>;

    fn zero() -> Self::Element {
        Poly::zero()
    }

    fn one() -> Self::Element {
        Poly::constant(R::one())
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
