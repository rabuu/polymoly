use crate::structures::{Field, PolyRing, Ring, Z};

pub trait EuclideanRing: Ring {
    fn euclidean_function(elem: Self::T) -> Option<usize>;
    fn euclidean_division(a: Self::T, b: Self::T) -> Option<(Self::T, Self::T)>;
}

impl EuclideanRing for Z {
    fn euclidean_function(elem: Self::T) -> Option<usize> {
        Some(elem.unsigned_abs())
    }

    fn euclidean_division(a: Self::T, b: Self::T) -> Option<(Self::T, Self::T)> {
        (b != 0).then_some((a.div_euclid(b), a.rem_euclid(b)))
    }
}

impl<F> EuclideanRing for PolyRing<F>
where
    F: Field,
    F::T: Clone + PartialEq,
{
    fn euclidean_function(elem: Self::T) -> Option<usize> {
        elem.deg()
    }

    fn euclidean_division(a: Self::T, b: Self::T) -> Option<(Self::T, Self::T)> {
        a.polynomial_division(b)
    }
}

pub fn extended_euclidean<E>(a: E::T, b: E::T) -> Option<(E::T, E::T, E::T)>
where
    E: EuclideanRing,
    E::T: Clone + PartialEq,
{
    if a == E::zero() && b == E::zero() {
        return None;
    }

    if b == E::zero() {
        return Some((a, E::one(), E::zero()));
    }

    let (_, rem) = E::euclidean_division(a.clone(), b.clone()).expect("b is non-zero");
    if rem == E::zero() {
        return Some((b, E::zero(), E::one()));
    }

    let (mut x, mut y) = (a, b);
    let (mut s1, mut s2) = (E::one(), E::zero());
    let (mut t1, mut t2) = (E::zero(), E::one());
    let (mut s, mut t) = (E::zero(), E::zero());

    loop {
        let (q, r) = E::euclidean_division(x.clone(), y.clone()).expect("y is non-zero");

        if r == E::zero() {
            break;
        }

        s = E::sub(s1, E::mul(q.clone(), s2.clone()));
        t = E::sub(t1, E::mul(q, t2.clone()));
        s1 = s2;
        s2 = s.clone();
        t1 = t2;
        t2 = t.clone();

        x = y;
        y = r;
    }

    Some((y, s, t))
}

pub fn extended_euclidean_int(a: isize, b: isize) -> Option<(usize, isize, isize)> {
    if a == 0 && b == 0 {
        return None;
    }

    if b == 0 {
        let s = if a > 0 { 1 } else { -1 };
        return Some((a.unsigned_abs(), s, 0));
    }

    if a % b == 0 {
        let t = if b > 0 { 1 } else { -1 };
        return Some((b.unsigned_abs(), 0, t));
    }

    let (mut x, mut y) = (a, b);
    let (mut s1, mut s2) = (1, 0);
    let (mut t1, mut t2) = (0, 1);
    let (mut s, mut t) = (0, 0);

    while x % y != 0 {
        let q = x.div_euclid(y);
        let r = x.rem_euclid(y);

        s = s1 - q * s2;
        t = t1 - q * t2;
        s1 = s2;
        s2 = s;
        t1 = t2;
        t2 = t;

        x = y;
        y = r;
    }

    Some((y as usize, s, t))
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
