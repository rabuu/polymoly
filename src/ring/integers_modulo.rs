use std::fmt;

use super::{Field, Ring};

pub(crate) trait IntegersModuloAny: Copy + fmt::Debug {
    fn n(&self) -> usize;
}

/// The ring `Z/nZ` of integers modulo `n`
#[derive(Clone, Copy)]
pub struct IntegersModuloN {
    n: usize,
}

impl IntegersModuloN {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl IntegersModuloAny for IntegersModuloN {
    fn n(&self) -> usize {
        self.n
    }
}

/// The field `Z/pZ` of integers modulo `p` where `p` is prime
///
/// This basically equivalent to [IntegersModuloN] but *must* only be used with a prime modulus.
/// That ensures the field properties of this ring.
#[derive(Clone, Copy)]
pub struct IntegersModuloP {
    p: usize,
}

impl IntegersModuloP {
    /// Construct the `Z/pZ` where `p` *must* be prime
    pub fn new_unchecked(p: usize) -> Self {
        Self { p }
    }

    /// Construct the `Z/pZ` and check if `p` is prime
    ///
    /// Return `None` if `p` is not prime
    pub fn new(p: usize) -> Option<Self> {
        is_prime(p).then(|| IntegersModuloP::new_unchecked(p))
    }
}

impl IntegersModuloAny for IntegersModuloP {
    fn n(&self) -> usize {
        self.p
    }
}

impl<T: IntegersModuloAny> Ring for T {
    type Element = isize;

    fn zero(&self) -> Self::Element {
        0
    }

    fn one(&self) -> Self::Element {
        1
    }

    fn add(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        self.id(lhs + rhs)
    }

    fn neg(&self, elem: Self::Element) -> Self::Element {
        self.id(-elem)
    }

    fn mul(&self, lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        self.id(lhs * rhs)
    }

    fn id(&self, elem: Self::Element) -> Self::Element {
        elem.rem_euclid(self.n() as isize)
    }
}

impl Field for IntegersModuloP {
    fn inv(&self, elem: Self::Element) -> Option<Self::Element> {
        let (_, s, _) = crate::euclid::extended_euclidean_int(elem, self.p as isize)?;
        Some(s.rem_euclid(self.p as isize))
    }
}

impl fmt::Debug for IntegersModuloN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Z/{}Z", self.n)
    }
}

impl fmt::Debug for IntegersModuloP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Z/{}Z", self.p)
    }
}

impl fmt::Display for IntegersModuloN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for IntegersModuloP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn is_prime(p: usize) -> bool {
    if p <= 1 {
        return false;
    }

    if p == 2 || p == 3 {
        return true;
    }

    if p % 2 == 0 || p % 3 == 0 {
        return false;
    }

    let sqrt = (p as f32).sqrt().ceil() as usize;
    for i in (5..=sqrt).step_by(6) {
        if p % i == 0 || p % (i + 2) == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_detection() {
        assert!(IntegersModuloP::new(0).is_none());
        assert!(IntegersModuloP::new(1).is_none());
        assert!(IntegersModuloP::new(4).is_none());
        assert!(IntegersModuloP::new(6).is_none());
        assert!(IntegersModuloP::new(8).is_none());
        assert!(IntegersModuloP::new(9).is_none());
        assert!(IntegersModuloP::new(333).is_none());
        assert!(IntegersModuloP::new(7909).is_none());

        assert!(IntegersModuloP::new(2).is_some());
        assert!(IntegersModuloP::new(3).is_some());
        assert!(IntegersModuloP::new(5).is_some());
        assert!(IntegersModuloP::new(7).is_some());
        assert!(IntegersModuloP::new(11).is_some());
        assert!(IntegersModuloP::new(13).is_some());
        assert!(IntegersModuloP::new(19).is_some());
        assert!(IntegersModuloP::new(43).is_some());
        assert!(IntegersModuloP::new(127).is_some());
        assert!(IntegersModuloP::new(7793).is_some());
    }
}
