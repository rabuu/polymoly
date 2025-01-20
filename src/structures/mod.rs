mod field;
mod ring;

use std::marker::PhantomData;

pub use field::Field;
pub use ring::{CommutativeRing, Ring};

#[derive(Debug)]
pub struct R;

#[derive(Debug)]
pub struct Z;

#[derive(Debug)]
pub struct ZMod<const N: usize>;

#[derive(Debug)]
pub struct PolyRing<R: CommutativeRing>(PhantomData<R>);
