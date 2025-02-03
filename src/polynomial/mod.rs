//! Polynomials over rings

pub mod display;
pub mod parse;

use std::ops;

use crate::ring::{Field, Ring};
use parse::ParsableRing;

/// A polynomial over the ring `R`
pub struct Polynomial<R: Ring> {
    ring: R,
    elems: Vec<R::Element>,
}

impl<R: Ring> Polynomial<R> {
    pub fn new(ring: R, elems: impl Into<Vec<R::Element>>) -> Self {
        let elems = elems.into().into_iter().map(|e| ring.id(e)).collect();
        let mut ret = Self { ring, elems };
        ret.cut_trailing_zeros();

        ret
    }

    pub fn with_capacity(ring: R, capacity: usize) -> Self {
        Self {
            ring,
            elems: Vec::with_capacity(capacity),
        }
    }

    pub fn zero(ring: R) -> Self {
        Self {
            ring,
            elems: vec![],
        }
    }

    fn zeros(ring: R, len: usize) -> Self {
        Self {
            ring,
            elems: vec![ring.zero(); len],
        }
    }

    pub fn constant(ring: R, constant: R::Element) -> Self {
        let mut ret = Self::new(ring, vec![ring.id(constant)]);
        ret.cut_trailing_zeros();

        ret
    }

    pub fn single(ring: R, elem: R::Element, deg: usize) -> Self {
        let mut elems = vec![ring.zero(); deg + 1];
        elems[deg] = elem;

        let mut ret = Self::new(ring, elems);
        ret.cut_trailing_zeros();

        ret
    }

    pub fn parse(ring: R, input: &str) -> Option<Self>
    where
        R: ParsableRing,
    {
        ring.parse_poly(input)
    }

    pub fn add_elem(&mut self, elem: R::Element, deg: usize) {
        self.fill_with_zeros(deg + 1);
        self.add_elem_unsafe(elem, deg);
        self.cut_trailing_zeros();
    }

    fn add_elem_unsafe(&mut self, elem: R::Element, deg: usize) {
        self.elems[deg] = self.ring.add(self.elems[deg].clone(), self.ring.id(elem));
    }

    pub fn deg(&self) -> Option<usize> {
        (!self.elems.is_empty()).then(|| self.elems.len() - 1)
    }

    pub fn lc(&self) -> R::Element {
        self.elems.last().cloned().unwrap_or(self.ring.zero())
    }

    pub fn is_zero(&self) -> bool {
        self.elems.is_empty()
    }

    fn fill_with_zeros(&mut self, new_len: usize) {
        if new_len > self.elems.len() {
            self.elems.resize_with(new_len, || self.ring.zero());
        }
    }

    fn cut_trailing_zeros(&mut self) {
        for _ in 0..self.elems.len() {
            if let Some(elem) = self.elems.last() {
                if *elem == self.ring.zero() {
                    self.elems.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl<F: Field> Polynomial<F> {
    pub fn polynomial_division(self, rhs: Polynomial<F>) -> Option<(Polynomial<F>, Polynomial<F>)> {
        if rhs.is_zero() {
            return None;
        }

        let ring = self.ring;
        let mut q = Polynomial::zeros(self.ring, self.elems.len());
        let mut r = self;
        let d = rhs.deg().expect("rhs is not zero");

        while !r.is_zero() {
            let r_deg = r.deg().expect("r is not zero");

            if r_deg < d {
                break;
            }

            let deg = r_deg - d;
            let quotient = ring.div(r.lc(), rhs.lc()).expect("rhs is not zero");
            let t = Polynomial::single(ring, quotient, deg);
            q += t.clone();
            r -= t * rhs.clone();
        }

        q.cut_trailing_zeros();
        r.cut_trailing_zeros();

        Some((q, r))
    }
}

impl<R: Ring> ops::Add<Polynomial<R>> for Polynomial<R> {
    type Output = Polynomial<R>;

    fn add(self, rhs: Polynomial<R>) -> Self::Output {
        let (longer, shorter) = if self.elems.len() > rhs.elems.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        let mut out = longer.clone();

        for (i, elem) in shorter.elems.into_iter().enumerate() {
            out.add_elem_unsafe(elem, i);
        }

        out.cut_trailing_zeros();
        out
    }
}

impl<R: Ring> ops::AddAssign<Polynomial<R>> for Polynomial<R> {
    fn add_assign(&mut self, rhs: Polynomial<R>) {
        self.fill_with_zeros(rhs.elems.len());
        for (i, elem) in rhs.elems.into_iter().enumerate() {
            self.add_elem_unsafe(elem, i);
        }
        self.cut_trailing_zeros();
    }
}

impl<R: Ring> ops::Neg for Polynomial<R> {
    type Output = Polynomial<R>;

    fn neg(self) -> Self::Output {
        let mut out = Self::with_capacity(self.ring, self.elems.len());
        for elem in self.elems {
            out.elems.push(self.ring.neg(elem));
        }
        out
    }
}

impl<R: Ring> ops::Sub<Polynomial<R>> for Polynomial<R> {
    type Output = Polynomial<R>;

    fn sub(mut self, rhs: Polynomial<R>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<R: Ring> ops::SubAssign<Polynomial<R>> for Polynomial<R> {
    fn sub_assign(&mut self, rhs: Polynomial<R>) {
        self.fill_with_zeros(rhs.elems.len());
        for (i, elem) in rhs.elems.into_iter().enumerate() {
            self.add_elem_unsafe(self.ring.neg(elem), i);
        }
        self.cut_trailing_zeros();
    }
}

impl<R: Ring> ops::Mul<Polynomial<R>> for Polynomial<R> {
    type Output = Polynomial<R>;

    fn mul(self, rhs: Polynomial<R>) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Polynomial::zero(self.ring);
        }

        let n = self.deg().unwrap();
        let m = rhs.deg().unwrap();
        let mut out = Self::zeros(self.ring, n + m + 1);

        for (i, a) in self.elems.iter().enumerate() {
            for (j, b) in rhs.elems.iter().enumerate() {
                out.add_elem_unsafe(self.ring.mul(a.clone(), b.clone()), i + j);
            }
        }

        out.cut_trailing_zeros();
        out
    }
}

impl<R: Ring> ops::MulAssign<Polynomial<R>> for Polynomial<R> {
    fn mul_assign(&mut self, rhs: Polynomial<R>) {
        let product = self.clone() * rhs;
        *self = product;
    }
}

impl<R: Ring> Clone for Polynomial<R> {
    fn clone(&self) -> Self {
        Self {
            ring: self.ring,
            elems: self.elems.clone(),
        }
    }
}

impl<R: Ring> PartialEq for Polynomial<R> {
    fn eq(&self, other: &Self) -> bool {
        self.elems == other.elems
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ring::Integers;

    #[test]
    fn degree() {
        assert_eq!(Polynomial::zero(Integers).deg(), None);
        assert_eq!(Polynomial::constant(Integers, 42).deg(), Some(0));

        for i in 0..3 {
            assert_eq!(Polynomial::single(Integers, 42, i).deg(), Some(i));
        }

        assert_eq!(Polynomial::single(Integers, 0, 9).deg(), None);

        let product = Polynomial::single(Integers, 2, 2) * Polynomial::single(Integers, 3, 3);
        assert_eq!(product.deg(), Some(5));
    }
}
