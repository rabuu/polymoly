mod ring;
mod field;

pub use ring::{Ring, CommutativeRing};
pub use field::Field;

#[derive(Debug)]
pub struct R;

#[derive(Debug)]
pub struct Z;

#[derive(Debug)]
pub struct ZMod<const N: usize>;
