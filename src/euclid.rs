//! Implementation of the (extended) euclidean algorithm

use crate::{Field, Integers, PolynomialRing, Ring};

/// A integral domain that has a well-defined euclidean division
pub trait EuclideanRing: Ring {
    fn euclidean_function(elem: Self::Element) -> Option<usize>;
    fn euclidean_division(
        a: Self::Element,
        b: Self::Element,
    ) -> Option<(Self::Element, Self::Element)>;
}

impl EuclideanRing for Integers {
    fn euclidean_function(elem: Self::Element) -> Option<usize> {
        Some(elem.unsigned_abs())
    }

    fn euclidean_division(
        a: Self::Element,
        b: Self::Element,
    ) -> Option<(Self::Element, Self::Element)> {
        (b != 0).then(|| (a.div_euclid(b), a.rem_euclid(b)))
    }
}

impl<F> EuclideanRing for PolynomialRing<F>
where
    F: Field,
    F::Element: Clone + PartialEq,
{
    fn euclidean_function(elem: Self::Element) -> Option<usize> {
        elem.deg()
    }

    fn euclidean_division(
        a: Self::Element,
        b: Self::Element,
    ) -> Option<(Self::Element, Self::Element)> {
        a.polynomial_division(b)
    }
}

/// Generalized extended euclidean algorithm (EEA)
///
/// This will compute an `Option<(gcd, s, t)>` so that `gcd` is the greatest common divisor of `a`
/// and `b` and the equation `(s * a) + (t * a) = gcd` holds.
///
/// This will be `None` iff `a` and `b` are 0.
pub fn extended_euclidean<E>(
    ring: E,
    a: E::Element,
    b: E::Element,
) -> Option<(E::Element, E::Element, E::Element)>
where
    E: EuclideanRing,
    E::Element: Clone + PartialEq,
{
    if a == ring.zero() && b == ring.zero() {
        return None;
    }

    if b == ring.zero() {
        return Some((a, ring.one(), ring.zero()));
    }

    let (_, rem) = E::euclidean_division(a.clone(), b.clone()).expect("b is non-zero");
    if rem == ring.zero() {
        return Some((b, ring.zero(), ring.one()));
    }

    let (mut x, mut y) = (a, b);
    let (mut s1, mut s2) = (ring.one(), ring.zero());
    let (mut t1, mut t2) = (ring.zero(), ring.one());
    let (mut s, mut t) = (ring.zero(), ring.zero());

    loop {
        let (q, r) = E::euclidean_division(x.clone(), y.clone()).expect("y is non-zero");

        if r == ring.zero() {
            break;
        }

        s = ring.sub(s1, ring.mul(q.clone(), s2.clone()));
        t = ring.sub(t1, ring.mul(q, t2.clone()));
        s1 = s2;
        s2 = s.clone();
        t1 = t2;
        t2 = t.clone();

        x = y;
        y = r;
    }

    Some((y, s, t))
}

/// Extended euclidean algorithm for integers
///
/// This is similar to [extended_euclidean] for [Integers] but always takes the positive GCD.
pub fn extended_euclidean_int(a: isize, b: isize) -> Option<(usize, isize, isize)> {
    extended_euclidean(Integers, a, b).map(|(gcd, s, t)| {
        if gcd < 0 {
            (gcd.unsigned_abs(), -s, -t)
        } else {
            (gcd.unsigned_abs(), s, t)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eea_48_neg30() {
        let (gcd, s, t) = extended_euclidean_int(48, -30).unwrap();
        assert_eq!((gcd, s, t), (6, 2, 3));
    }
}
