//! # PolyMoly
//! A library for polynomials over abstract rings.

pub mod euclid;
mod parse;
mod polynomial;
pub mod ring;

pub use polynomial::Poly;
pub use ring::{integer::Z, poly_ring::PolyRing, real::R, zmod::{ZModN, ZModP}};
pub use ring::{Field, Ring};
