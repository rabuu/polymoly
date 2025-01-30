//! # PolyMoly
//! A library for polynomials over abstract rings.

pub mod euclid;
pub mod polynomial;
pub mod ring;

pub use polynomial::Polynomial;
pub use ring::{
    integers::Integers,
    integers_modulo::{IntegersModuloN, IntegersModuloP},
    polynomial_ring::PolynomialRing,
    reals::Reals,
};
pub use ring::{Field, Ring};
