use std::{fmt, ops};

use crate::structures::{Field, Ring};

pub struct Poly<R: Ring>(Vec<R::Element>);

impl<R: Ring> Poly<R> {
    pub fn new(elems: impl Into<Vec<R::Element>>) -> Self {
        let elems = elems.into().into_iter().map(R::id).collect();
        Self(elems)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn zero() -> Self {
        Self(vec![])
    }

    fn zeros(len: usize) -> Self
    where
        R::Element: Clone,
    {
        Self(vec![R::zero(); len])
    }

    pub fn constant(constant: R::Element) -> Self {
        Self(vec![R::id(constant)])
    }

    pub fn single(elem: R::Element, deg: usize) -> Self
    where
        R::Element: Clone,
    {
        let mut elems = vec![R::zero(); deg + 1];
        elems[deg] = elem;
        Self(elems)
    }

    pub fn add_elem(&mut self, elem: R::Element, deg: usize)
    where
        R::Element: Clone + PartialEq,
    {
        self.fill_with_zeros(deg + 1);
        self.add_elem_unsafe(elem, deg);
        self.restore_length();
    }

    fn add_elem_unsafe(&mut self, elem: R::Element, deg: usize)
    where
        R::Element: Clone,
    {
        self.0[deg] = R::add(self.0[deg].clone(), R::id(elem));
    }

    pub fn deg(&self) -> Option<usize> {
        (!self.0.is_empty()).then_some(self.0.len())
    }

    pub fn lc(&self) -> R::Element
    where
        R::Element: Clone,
    {
        self.0.last().cloned().unwrap_or(R::zero())
    }

    pub fn is_zero(&self) -> bool
    where
        R::Element: PartialEq,
    {
        self.0.is_empty()
    }

    fn fill_with_zeros(&mut self, new_len: usize) {
        if new_len > self.0.len() {
            self.0.resize_with(new_len, || R::zero());
        }
    }

    fn restore_length(&mut self)
    where
        R::Element: PartialEq,
    {
        for _ in 0..self.0.len() {
            if let Some(elem) = self.0.last() {
                if *elem == R::zero() {
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

impl<F: Field> Poly<F> {
    pub fn polynomial_division(self, rhs: Poly<F>) -> Option<(Poly<F>, Poly<F>)>
    where
        F::Element: Clone + PartialEq,
    {
        if rhs.is_zero() {
            return None;
        }

        let mut q = Poly::<F>::zeros(self.0.len());
        let mut r = self;
        let d = rhs.deg().expect("rhs is not zero");

        while !r.is_zero() {
            let r_deg = r.deg().expect("r is not zero");

            if r_deg < d {
                break;
            }

            let deg = r_deg - d;
            let quotient = F::div(r.lc(), rhs.lc()).expect("rhs is not zero");
            let t = Poly::single(quotient, deg);
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

impl<R> ops::AddAssign<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    fn add_assign(&mut self, rhs: Poly<R>) {
        self.fill_with_zeros(rhs.0.len());
        for (i, elem) in rhs.0.into_iter().enumerate() {
            self.add_elem_unsafe(elem, i);
        }
        self.restore_length();
    }
}

impl<R> ops::Neg for Poly<R>
where
    R: Ring,
{
    type Output = Poly<R>;

    fn neg(self) -> Self::Output {
        let mut out = Self::with_capacity(self.0.len());
        for elem in self.0 {
            out.0.push(R::neg(elem));
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

impl<R> ops::SubAssign<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    fn sub_assign(&mut self, rhs: Poly<R>) {
        self.fill_with_zeros(rhs.0.len());
        for (i, elem) in rhs.0.into_iter().enumerate() {
            self.add_elem_unsafe(R::neg(elem), i);
        }
        self.restore_length();
    }
}

impl<R> ops::Mul<Poly<R>> for Poly<R>
where
    R: Ring,
    R::Element: Clone + PartialEq,
{
    type Output = Poly<R>;

    fn mul(self, rhs: Poly<R>) -> Self::Output {
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

impl<R> Default for Poly<R>
where
    R: Ring,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<R> Clone for Poly<R>
where
    R: Ring,
    R::Element: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<R> PartialEq for Poly<R>
where
    R: Ring,
    R::Element: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<R> fmt::Debug for Poly<R>
where
    R: Ring,
    R::Element: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Poly").field(&self.0).finish()
    }
}

trait SimpleDisplay: fmt::Display {}
impl SimpleDisplay for f64 {}
impl SimpleDisplay for isize {}
impl SimpleDisplay for usize {}

impl<R> fmt::Display for Poly<R>
where
    R: Ring,
    R::Element: SimpleDisplay + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::with_capacity(self.0.len() * 3);
        for (i, elem) in self.0.iter().enumerate().rev() {
            if *elem == R::zero() {
                continue;
            }

            if i < self.0.len() - 1 {
                str.push_str(" + ");
            }

            let elem = (*elem != R::one() || i == 0).then_some(elem);
            let elem_str = elem.map(|e| format!("{e}")).unwrap_or_default();

            let x = match i {
                0 => "".to_string(),
                1 => "x".to_string(),
                _ => format!("x^{i}"),
            };
            str.push_str(&format!("{elem_str}{x}"));
        }

        if str.is_empty() {
            str = format!("{}", R::zero());
        }

        write!(f, "{str}")
    }
}
