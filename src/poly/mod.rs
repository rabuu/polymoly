//! Polynomials over rings

pub mod display;
pub mod parse;

use std::ops;

use crate::ring::{Field, Ring};
use parse::ParsableRing;

/// A polynomial over the ring `R`
pub struct Poly<R: Ring> {
    ring: R,
    elems: Vec<R::Element>,
}

impl<R: Ring> Poly<R> {
    pub fn new(ring: R, elems: impl Into<Vec<R::Element>>) -> Self
    where
        R::Element: PartialEq,
    {
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

    fn zeros(ring: R, len: usize) -> Self
    where
        R::Element: Clone,
    {
        Self {
            ring,
            elems: vec![ring.zero(); len],
        }
    }

    pub fn constant(ring: R, constant: R::Element) -> Self
    where
        R::Element: PartialEq,
    {
        let mut ret = Self::new(ring, vec![ring.id(constant)]);
        ret.cut_trailing_zeros();

        ret
    }

    pub fn single(ring: R, elem: R::Element, deg: usize) -> Self
    where
        R::Element: Clone + PartialEq,
    {
        let mut elems = vec![ring.zero(); deg + 1];
        elems[deg] = elem;

        let mut ret = Self::new(ring, elems);
        ret.cut_trailing_zeros();

        ret
    }

    pub fn parse(ring: R, input: &str) -> Option<Self>
    where
        R: ParsableRing,
        R::Element: Clone + PartialEq,
    {
        ring.parse_poly(input)
    }

    pub fn add_elem(&mut self, elem: R::Element, deg: usize)
    where
        R::Element: Clone + PartialEq,
    {
        self.fill_with_zeros(deg + 1);
        self.add_elem_unsafe(elem, deg);
        self.cut_trailing_zeros();
    }

    fn add_elem_unsafe(&mut self, elem: R::Element, deg: usize)
    where
        R::Element: Clone,
    {
        self.elems[deg] = self.ring.add(self.elems[deg].clone(), self.ring.id(elem));
    }

    pub fn deg(&self) -> Option<usize> {
        (!self.elems.is_empty()).then(|| self.elems.len() - 1)
    }

    pub fn lc(&self) -> R::Element
    where
        R::Element: Clone,
    {
        self.elems.last().cloned().unwrap_or(self.ring.zero())
    }

    pub fn is_zero(&self) -> bool
    where
        R::Element: PartialEq,
    {
        self.elems.is_empty()
    }

    fn fill_with_zeros(&mut self, new_len: usize) {
        if new_len > self.elems.len() {
            self.elems.resize_with(new_len, || self.ring.zero());
        }
    }

    fn cut_trailing_zeros(&mut self)
    where
        R::Element: PartialEq,
    {
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

impl<F: Field> Poly<F> {
    pub fn polynomial_division(self, rhs: Poly<F>) -> Option<(Poly<F>, Poly<F>)>
    where
        F::Element: Clone + PartialEq,
    {
        if rhs.is_zero() {
            return None;
        }

        let ring = self.ring;
        let mut q = Poly::zeros(self.ring, self.elems.len());
        let mut r = self;
        let d = rhs.deg().expect("rhs is not zero");

        while !r.is_zero() {
            let r_deg = r.deg().expect("r is not zero");

            if r_deg < d {
                break;
            }

            let deg = r_deg - d;
            let quotient = ring.div(r.lc(), rhs.lc()).expect("rhs is not zero");
            let t = Poly::single(ring, quotient, deg);
            q += t.clone();
            r -= t * rhs.clone();
        }

        Some((q, r))
    }
}

impl<R> ops::Add<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    type Output = Poly<R>;

    fn add(self, rhs: Poly<R>) -> Self::Output {
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

impl<R> ops::AddAssign<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    fn add_assign(&mut self, rhs: Poly<R>) {
        self.fill_with_zeros(rhs.elems.len());
        for (i, elem) in rhs.elems.into_iter().enumerate() {
            self.add_elem_unsafe(elem, i);
        }
        self.cut_trailing_zeros();
    }
}

impl<R> ops::Neg for Poly<R>
where
    R: Ring,
{
    type Output = Poly<R>;

    fn neg(self) -> Self::Output {
        let mut out = Self::with_capacity(self.ring, self.elems.len());
        for elem in self.elems {
            out.elems.push(self.ring.neg(elem));
        }
        out
    }
}

impl<R> ops::Sub<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    type Output = Poly<R>;

    fn sub(self, rhs: Poly<R>) -> Self::Output {
        let ring = self.ring;

        let (longer, shorter) = if self.elems.len() > rhs.elems.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        let mut out = longer.clone();

        for (i, elem) in shorter.elems.into_iter().enumerate() {
            out.add_elem_unsafe(ring.neg(elem), i);
        }

        out.cut_trailing_zeros();
        out
    }
}

impl<R> ops::SubAssign<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    fn sub_assign(&mut self, rhs: Poly<R>) {
        self.fill_with_zeros(rhs.elems.len());
        for (i, elem) in rhs.elems.into_iter().enumerate() {
            self.add_elem_unsafe(self.ring.neg(elem), i);
        }
        self.cut_trailing_zeros();
    }
}

impl<R> ops::Mul<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    type Output = Poly<R>;

    fn mul(self, rhs: Poly<R>) -> Self::Output {
        if self.is_zero() || rhs.is_zero() {
            return Poly::zero(self.ring);
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

impl<R> ops::MulAssign<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    fn mul_assign(&mut self, rhs: Poly<R>) {
        let product = self.clone() * rhs;
        *self = product;
    }
}

impl<R> Clone for Poly<R>
where
    R: Ring,
    R::Element: Clone,
{
    fn clone(&self) -> Self {
        Self {
            ring: self.ring,
            elems: self.elems.clone(),
        }
    }
}

impl<R> PartialEq for Poly<R>
where
    R: Ring,
    R::Element: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.elems == other.elems
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Z;

    #[test]
    fn degree() {
        assert_eq!(Poly::zero(Z).deg(), None);
        assert_eq!(Poly::constant(Z, 42).deg(), Some(0));

        for i in 0..3 {
            assert_eq!(Poly::single(Z, 42, i).deg(), Some(i));
        }

        assert_eq!(Poly::single(Z, 0, 9).deg(), None);

        let product = Poly::single(Z, 2, 2) * Poly::single(Z, 3, 3);
        assert_eq!(product.deg(), Some(5));
    }
}
