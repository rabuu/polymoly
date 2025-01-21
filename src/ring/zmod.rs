//! The ring of integers modulo `n` `Z/nZ`

use std::fmt;

use super::{Field, Ring};

pub(crate) trait ZMod: Copy {
    fn n(&self) -> usize;
}

/// The ring of integers modulo `n` `Z/nZ`
#[derive(Clone, Copy)]
pub struct ZModN {
    n: usize,
}

impl ZModN {
    pub fn new(n: usize) -> Self {
        Self { n }
    }
}

impl ZMod for ZModN {
    fn n(&self) -> usize {
        self.n
    }
}

/// The field of integers modulo `p` where `p` is prime `Z/pZ`
#[derive(Clone, Copy)]
pub struct ZModP {
    p: usize,
}

impl ZModP {
    /// Construct the `Z/pZ` where `p` *must* be prime
    pub fn new(p: usize) -> Self {
        Self { p }
    }

    /// Construct the `Z/pZ` and check if `p` is prime
    ///
    /// Return `None` if `p` is not prime
    pub fn checked_new(p: usize) -> Option<Self> {
        if p <= 1 {
            return None;
        }

        if p == 2 || p == 3 {
            return Some(Self::new(p));
        }

        if p % 2 == 0 || p % 3 == 0 {
            return None;
        }

        let sqrt = (p as f32).sqrt().ceil() as usize;
        for i in (5..=sqrt).step_by(6) {
            if p % i == 0 || p % (i + 2) == 0 {
                return None;
            }
        }

        Some(Self::new(p))
    }
}

impl ZMod for ZModP {
    fn n(&self) -> usize {
        self.p
    }
}


impl<T: ZMod> Ring for T {
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

impl Field for ZModP {
    fn inv(&self, elem: Self::Element) -> Option<Self::Element> {
        let (_, s, _) = crate::euclid::extended_euclidean_int(elem, self.p as isize)?;
        Some(s.rem_euclid(self.p as isize))
    }
}

impl fmt::Debug for ZModN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Z/{}Z", self.n)
    }
}

impl fmt::Debug for ZModP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Z/{}Z", self.p)
    }
}

impl fmt::Display for ZModN {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ZModP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_detection() {
        assert!(ZModP::checked_new(0).is_none());
        assert!(ZModP::checked_new(1).is_none());
        assert!(ZModP::checked_new(4).is_none());
        assert!(ZModP::checked_new(6).is_none());
        assert!(ZModP::checked_new(8).is_none());
        assert!(ZModP::checked_new(9).is_none());
        assert!(ZModP::checked_new(333).is_none());
        assert!(ZModP::checked_new(7909).is_none());

        assert!(ZModP::checked_new(2).is_some());
        assert!(ZModP::checked_new(3).is_some());
        assert!(ZModP::checked_new(5).is_some());
        assert!(ZModP::checked_new(7).is_some());
        assert!(ZModP::checked_new(11).is_some());
        assert!(ZModP::checked_new(13).is_some());
        assert!(ZModP::checked_new(19).is_some());
        assert!(ZModP::checked_new(43).is_some());
        assert!(ZModP::checked_new(127).is_some());
        assert!(ZModP::checked_new(7793).is_some());
    }
}
