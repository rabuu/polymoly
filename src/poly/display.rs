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

pub struct DisplayPart<R: DisplayRing> {
    pub coefficient: Option<R::Element>,
    pub variable: Option<Option<usize>>,
}

impl<R> DisplayPart<R>
where 
    R: DisplayRing,
    R::Element: Clone + PartialEq,
{
    pub fn get_parts(poly: &Poly<R>) -> Vec<DisplayPart<R>> {
        if poly.is_zero() {
            return vec![
                DisplayPart {
                    coefficient: Some(poly.ring.zero()),
                    variable: None,
                }
            ];
        }

        let mut parts = Vec::new();
        for (i, elem) in poly.elems.iter().enumerate().rev() {
            if *elem == poly.ring.zero() {
                continue;
            }

            let coefficient = (*elem != poly.ring.one() || i == 0).then_some(elem.clone());

            let variable = match i {
                0 => None,
                1 => Some(None),
                _ => Some(Some(i)),
            };

            parts.push(DisplayPart { coefficient, variable });
        }

        parts
    }
}

impl<R> fmt::Display for Poly<R>
where
    R: DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts = DisplayPart::get_parts(self);
        let len = parts.len();

        let string = parts.into_iter().enumerate().fold(String::new(), |mut acc, (i, part)| {
            if let Some(e) = part.coefficient {
                acc.push_str(&e.to_string());
            }

            if let Some(variable) = part.variable {
                acc.push('x');
                if let Some(exponent) = variable {
                    acc.push('^');
                    acc.push_str(&exponent.to_string());
                }
            }

            if i < len - 1 {
                acc.push_str(" + ");
            }

            acc
        });

        write!(f, "{string}")
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
