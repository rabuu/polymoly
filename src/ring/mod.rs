//! Algebraic (commutative) rings

mod integers;
mod integers_modulo;
mod polynomial_ring;
mod reals;

pub use integers::Integers;
pub use integers_modulo::{IntegersModuloN, IntegersModuloP};
pub use polynomial_ring::PolynomialRing;
pub use reals::Reals;

pub(crate) use integers_modulo::IntegersModuloAny;

/// An algebraic commutative ring
pub trait Ring: Copy + std::fmt::Debug {
    type Element: Clone + PartialEq;

    fn zero(&self) -> Self::Element;
    fn one(&self) -> Self::Element;

    fn add(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element;
    fn neg(&self, elem: Self::Element) -> Self::Element;
    fn mul(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element;

    fn sub(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        self.add(lhs, self.neg(rhs))
    }

    fn id(&self, elem: Self::Element) -> Self::Element {
        elem
    }
}

/// An algebraic field
pub trait Field: Ring {
    fn inv(&self, elem: Self::Element) -> Option<Self::Element>;

    fn div(&self, lhs: Self::Element, rhs: Self::Element) -> Option<Self::Element> {
        self.inv(rhs).map(|inv| self.mul(lhs, inv))
    }
}
