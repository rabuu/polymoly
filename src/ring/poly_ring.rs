use std::marker::PhantomData;

use super::Ring;
use crate::Poly;

#[derive(Debug)]
pub struct PolyRing<R: Ring>(PhantomData<R>);

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
