use crate::Poly;

use super::{PolyRing, ZMod, R, Z};

/// A commutative ring
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
    R: Ring,
    R::T: Clone + PartialEq,
{
    type T = Poly<R>;

    fn zero() -> Self::T {
        Poly::default()
    }

    fn one() -> Self::T {
        Poly::constant(R::one())
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
