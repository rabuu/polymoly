//! The ring of polynomials `R[x]`

use super::Ring;
use crate::Poly;

/// The ring of polynomials `R[x]` where `R` is another ring
#[derive(Debug, Clone, Copy)]
pub struct PolyRing<R: Ring> {
    ring: R,
}

impl<R: Ring> PolyRing<R> {
    pub fn new(ring: R) -> Self {
        Self { ring }
    }
}

impl<R> Ring for PolyRing<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    type Element = Poly<R>;

    fn zero(&self) -> Self::Element {
        Poly::zero(self.ring)
    }

    fn one(&self) -> Self::Element {
        Poly::constant(self.ring, self.ring.one())
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
