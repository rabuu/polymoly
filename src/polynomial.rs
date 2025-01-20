use std::{fmt, ops};

use crate::alg::CommutativeRing;

pub struct Polynomial<R: CommutativeRing>(Vec<R::T>);

impl<R: CommutativeRing> Polynomial<R> {
    pub const ZERO: Self = Self(Vec::new());

    pub fn new() -> Self {
        Self::ZERO
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn zeros(len: usize) -> Self
    where
        R::T: Clone,
    {
        Self(vec![R::ZERO; len])
    }

    pub fn constant(constant: R::T) -> Self {
        Self(vec![R::id(constant)])
    }

    pub fn add_elem(&mut self, elem: R::T, deg: usize)
    where
        R::T: Clone + PartialEq,
    {
        self.fill_with_zeros(deg + 1);
        self.add_elem_unsafe(elem, deg);
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

    fn fill_with_zeros(&mut self, new_len: usize) {
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
        self.fill_with_zeros(rhs.0.len());
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
        self.fill_with_zeros(rhs.0.len());
        for (i, elem) in rhs.0.into_iter().enumerate() {
            self.add_elem_unsafe(R::neg(elem), i);
        }
        self.restore_length();
    }
}

impl<R> ops::Mul<Polynomial<R>> for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    type Output = Polynomial<R>;

    fn mul(self, rhs: Polynomial<R>) -> Self::Output {
        let n = self.deg().unwrap_or(0);
        let m = rhs.deg().unwrap_or(0);
        let mut out = Self::zeros(n + m + 1);

        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in rhs.0.iter().enumerate() {
                out.add_elem_unsafe(R::mul(a.clone(), b.clone()), i + j);
            }
        }

        out.restore_length();
        out
    }
}

impl<R> ops::MulAssign<Polynomial<R>> for Polynomial<R>
where
    R: CommutativeRing,
    R::T: Clone + PartialEq,
{
    fn mul_assign(&mut self, rhs: Polynomial<R>) {
        let product = self.clone() * rhs;
        *self = product;
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

impl<R> fmt::Debug for Polynomial<R>
where
    R: CommutativeRing,
    R::T: fmt::Debug,
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

impl<R> PartialEq for Polynomial<R>
where
    R: CommutativeRing,
    R::T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
