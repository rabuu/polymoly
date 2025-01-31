use crate::polynomial::Polynomial;

use super::Ring;

/// The ring `R[x]` of polynomials where `R` is another ring
#[derive(Debug, Clone, Copy)]
pub struct PolynomialRing<R: Ring> {
    ring: R,
}

impl<R: Ring> PolynomialRing<R> {
    pub fn new(ring: R) -> Self {
        Self { ring }
    }
}

impl<R: Ring> Ring for PolynomialRing<R> {
    type Element = Polynomial<R>;

    fn zero(&self) -> Self::Element {
        Polynomial::zero(self.ring)
    }

    fn one(&self) -> Self::Element {
        Polynomial::constant(self.ring, self.ring.one())
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
