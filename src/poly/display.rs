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

/// A part of a displayable [Poly], useful for outputting
///
/// - `coefficient` can be `None` (if it is a One) or `Some` otherwise
/// - `variable` can be `None` (degree 0), `Some(None)` (degree 1) or `Some(pot)` (degree `pot`)
///
/// See [Self::get_parts] for how to generate these parts from a [Poly].
pub struct DisplayPart<R: DisplayRing> {
    pub coefficient: Option<R::Element>,
    pub variable: Option<Option<usize>>,
}

impl<R> DisplayPart<R>
where
    R: DisplayRing,
    R::Element: Clone + PartialEq,
{
    /// Generate the [DisplayPart]s from a [Poly]
    pub fn get_parts(poly: &Poly<R>) -> Vec<DisplayPart<R>> {
        if poly.is_zero() {
            return vec![DisplayPart {
                coefficient: Some(poly.ring.zero()),
                variable: None,
            }];
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

            parts.push(DisplayPart {
                coefficient,
                variable,
            });
        }

        parts
    }
}

/// Fold a displayable [Poly] into another type.
///
/// This uses [DisplayPart]s for "rendering".
///
/// - `init`: the seed value of the folding
/// - `coeff`: if a coefficient can be displayed, do something to the accumulator
/// - `var`: if a variable can be displayed, do something to the accumulator
/// - `exp`: if a exponent can be displayed, do something to the accumulator
/// - `sep`: do something to the accumulator to add a seperator
pub fn fold_displayring_poly<R, O, C, V, E, S>(
    poly: &Poly<R>,
    init: O,
    coeff: C,
    var: V,
    exp: E,
    sep: S,
) -> O
where
    R: DisplayRing,
    R::Element: Clone + PartialEq,
    C: Fn(&mut O, R::Element),
    V: Fn(&mut O),
    E: Fn(&mut O, usize),
    S: Fn(&mut O),
{
    let parts = DisplayPart::get_parts(poly);
    let len = parts.len();

    parts
        .into_iter()
        .enumerate()
        .fold(init, |mut acc, (i, part)| {
            if let Some(c) = part.coefficient {
                coeff(&mut acc, c);
            }

            if let Some(variable) = part.variable {
                var(&mut acc);
                if let Some(exponent) = variable {
                    exp(&mut acc, exponent);
                }
            }

            if i < len - 1 {
                sep(&mut acc);
            }

            acc
        })
}

impl<R> fmt::Display for Poly<R>
where
    R: DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = fold_displayring_poly(
            self,
            String::new(),
            |s, coeff| {
                s.push_str(&coeff.to_string());
            },
            |s| {
                s.push('x');
            },
            |s, exp| {
                s.push('^');
                s.push_str(&exp.to_string());
            },
            |s| {
                s.push_str(" + ");
            },
        );

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
