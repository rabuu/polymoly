//! # PolyMoly
//! A library for polynomials over abstract rings.

pub mod euclid;
pub mod poly;
pub mod ring;

pub use poly::Poly;
pub use ring::{
    integer::Z,
    poly_ring::PolyRing,
    real::R,
    zmod::{ZModN, ZModP},
};
pub use ring::{Field, Ring};
