pub trait Ring {
    type T;

    const ZERO: Self::T;
    const ONE: Self::T;

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
pub trait Field: CommutativeRing {}

pub struct R;

impl Ring for R {
    type T = f64;

    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;

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

impl CommutativeRing for R {}
impl Field for R {}

pub struct Z;

impl Ring for Z {
    type T = isize;

    const ZERO: isize = 0;
    const ONE: isize = 1;

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

impl CommutativeRing for Z {}

pub struct ZMod<const N: usize>;

impl<const N: usize> Ring for ZMod<N> {
    type T = usize;

    const ZERO: usize = 0;
    const ONE: usize = 1;

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

impl<const N: usize> CommutativeRing for ZMod<N> {}
