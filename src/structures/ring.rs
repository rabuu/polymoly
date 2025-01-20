use crate::Polynomial;

use super::{PolyRing, ZMod, R, Z};

pub trait Ring {
    type T;

    fn zero() -> Self::T;
    fn one() -> Self::T;

    fn add(lhs: Self::T, rhs: Self::T) -> Self::T;
    fn neg(elem: Self::T) -> Self::T;
    fn mul(lhs: Self::T, rhs: Self::T) -> Self::T;

    fn sub(lhs: Self::T, rhs: Self::T) -> Self::T {
        Self::add(lhs, Self::neg(rhs))
    }

    fn id(elem: Self::T) -> Self::T {
        elem
    }
}

pub trait CommutativeRing: Ring {}

impl Ring for R {
    type T = f64;

    fn zero() -> Self::T {
        0.0
    }

    fn one() -> Self::T {
        1.0
    }

    fn add(lhs: f64, rhs: f64) -> f64 {
        lhs + rhs
    }

    fn neg(elem: f64) -> f64 {
        -elem
    }

    fn mul(lhs: f64, rhs: f64) -> f64 {
        lhs * rhs
    }
}

impl Ring for Z {
    type T = isize;

    fn zero() -> Self::T {
        0
    }

    fn one() -> Self::T {
        1
    }

    fn add(lhs: isize, rhs: isize) -> isize {
        lhs + rhs
    }

    fn neg(elem: isize) -> isize {
        -elem
    }

    fn mul(lhs: isize, rhs: isize) -> isize {
        lhs * rhs
    }
}

impl<const N: usize> Ring for ZMod<N> {
    type T = usize;

    fn zero() -> Self::T {
        0
    }

    fn one() -> Self::T {
        1
    }

    fn add(lhs: usize, rhs: usize) -> usize {
        (lhs + rhs) % N
    }

    fn neg(elem: usize) -> usize {
        let negative: isize = -(elem as isize);
        (negative.rem_euclid(N as isize)) as usize
    }

    fn mul(lhs: usize, rhs: usize) -> usize {
        (lhs * rhs) % N
    }

    fn id(elem: usize) -> usize {
        elem % N
    }
}

impl<R> Ring for PolyRing<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    type T = Polynomial<R>;

    fn zero() -> Self::T {
        Polynomial::default()
    }

    fn one() -> Self::T {
        Polynomial::constant(R::one())
    }

    fn add(lhs: Self::T, rhs: Self::T) -> Self::T {
        lhs + rhs
    }

    fn neg(elem: Self::T) -> Self::T {
        -elem
    }

    fn mul(lhs: Self::T, rhs: Self::T) -> Self::T {
        lhs * rhs
    }
}

impl CommutativeRing for R {}
impl CommutativeRing for Z {}
impl<const N: usize> CommutativeRing for ZMod<N> {}

impl<R> CommutativeRing for PolyRing<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
}
