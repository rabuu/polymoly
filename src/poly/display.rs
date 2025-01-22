//! Displaying polynomials

use std::fmt;

use crate::ring::zmod::ZMod;
use crate::{Ring, R, Z};

use super::Poly;

/// A ring where polynomials can be displayed
pub trait DisplayRing: Ring {}
impl DisplayRing for R {}
impl DisplayRing for Z {}
impl<T: ZMod> DisplayRing for T {}

impl<R> fmt::Display for Poly<R>
where
    R: DisplayRing,
    R::Element: PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::with_capacity(self.elems.len() * 3);
        for (i, elem) in self.elems.iter().enumerate().rev() {
            if *elem == self.ring.zero() {
                continue;
            }

            if i < self.elems.len() - 1 {
                str.push_str(" + ");
            }

            let elem = (*elem != self.ring.one() || i == 0).then_some(elem);
            let elem_str = elem.map(|e| format!("{e}")).unwrap_or_default();

            let x = match i {
                0 => "".to_string(),
                1 => "x".to_string(),
                _ => format!("x^{i}"),
            };
            str.push_str(&format!("{elem_str}{x}"));
        }

        if str.is_empty() {
            str = format!("{}", self.ring.zero());
        }

        write!(f, "{str}")
    }
}

impl<R> fmt::Debug for Poly<R>
where
    R: Ring + fmt::Debug,
    R::Element: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Poly")
            .field("ring", &self.ring)
            .field("elems", &self.elems)
            .finish()
    }
}
