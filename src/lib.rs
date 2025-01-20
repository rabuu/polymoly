use std::fmt::Debug;
use std::ops;

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

pub struct Polynomial<R: CommutativeRing>(Vec<R::T>);

impl<R: CommutativeRing> Polynomial<R> {
    pub const ZERO: Self = Self(Vec::new());

    pub fn new() -> Self {
        Self::ZERO
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn constant(constant: R::T) -> Self {
        Self(vec![R::id(constant)])
    }

    pub fn add_elem(&mut self, elem: R::T, deg: usize)
    where
        R::T: Clone + PartialEq,
    {
        self.resize(deg + 1);
        self.0[deg] = R::add(self.0[deg].clone(), R::id(elem));
        self.restore_length();
    }

    fn add_elem_unsafe(&mut self, elem: R::T, deg: usize)
    where
        R::T: Clone,
    {
        self.0[deg] = R::add(self.0[deg].clone(), R::id(elem));
    }

    pub fn deg(&self) -> Option<usize> {
        (!self.0.is_empty()).then_some(self.0.len())
    }

    pub fn lc(&self) -> R::T
    where
        R::T: Clone,
    {
        self.0.last().cloned().unwrap_or(R::ZERO)
    }

    fn resize(&mut self, new_len: usize) {
        if new_len > self.0.len() {
            self.0.resize_with(new_len, || R::ZERO);
        }
    }

    fn restore_length(&mut self)
    where
        R::T: PartialEq,
    {
        for _ in 0..self.0.len() {
            if let Some(elem) = self.0.last() {
                if *elem == R::ZERO {
                    self.0.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl<R> Default for Polynomial<R>
where
    R: CommutativeRing,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Debug for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Polynomial").field(&self.0).finish()
    }
}

impl<R> Clone for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R> ops::Add<Polynomial<R>> for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    type Output = Polynomial<R>;

    fn add(self, rhs: Polynomial<R>) -> Self::Output {
        let (longer, shorter) = if self.0.len() > rhs.0.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        let mut out = longer.clone();

        for (i, elem) in shorter.0.into_iter().enumerate() {
            out.add_elem_unsafe(elem, i);
        }

        out.restore_length();
        out
    }
}

impl<R> ops::AddAssign<Polynomial<R>> for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    fn add_assign(&mut self, rhs: Polynomial<R>) {
        self.resize(rhs.0.len());
        for (i, elem) in rhs.0.into_iter().enumerate() {
            self.add_elem_unsafe(elem, i);
        }
        self.restore_length();
    }
}

impl<R> ops::Neg for Polynomial<R>
where
    R: CommutativeRing,
{
    type Output = Polynomial<R>;

    fn neg(self) -> Self::Output {
        let mut out = Self::with_capacity(self.0.len());
        for elem in self.0 {
            out.0.push(R::neg(elem));
        }
        out
    }
}

impl<R> ops::Sub<Polynomial<R>> for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    type Output = Polynomial<R>;

    fn sub(self, rhs: Polynomial<R>) -> Self::Output {
        let (longer, shorter) = if self.0.len() > rhs.0.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        let mut out = longer.clone();

        for (i, elem) in shorter.0.into_iter().enumerate() {
            out.add_elem_unsafe(R::neg(elem), i);
        }

        out.restore_length();
        out
    }
}

impl<R> ops::SubAssign<Polynomial<R>> for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    fn sub_assign(&mut self, rhs: Polynomial<R>) {
        self.resize(rhs.0.len());
        for (i, elem) in rhs.0.into_iter().enumerate() {
            self.add_elem_unsafe(R::neg(elem), i);
        }
        self.restore_length();
    }
}
