//! Displaying polynomials

use std::fmt;

use crate::ring::integers_modulo::IntegersModuloAny;
use crate::{Integers, Reals, Ring};

use super::Polynomial;

/// A ring where polynomials can be displayed
pub trait DisplayRing: Ring {}
impl DisplayRing for Reals {}
impl DisplayRing for Integers {}
impl<T: IntegersModuloAny> DisplayRing for T {}

/// A part of a displayable [Polynomial], useful for outputting
///
/// - `coefficient` can be `None` (if it is a One) or `Some` otherwise
/// - `variable` can be `None` (degree 0), `Some(None)` (degree 1) or `Some(pot)` (degree `pot`)
///
/// See [Self::get_parts] for how to generate these parts from a [Polynomial].
pub struct DisplayPart<R: DisplayRing> {
    pub coefficient: Option<R::Element>,
    pub variable: Option<Option<usize>>,
}

impl<R> DisplayPart<R>
where
    R: DisplayRing,
    R::Element: Clone + PartialEq,
{
    /// Generate the [DisplayPart]s from a [Polynomial]
    pub fn get_parts(poly: &Polynomial<R>) -> Vec<DisplayPart<R>> {
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

impl<R> Polynomial<R>
where
    R: DisplayRing,
    R::Element: Clone + PartialEq,
{
    /// Fold a displayable [Polynomial] into another type.
    ///
    /// This uses [DisplayPart]s for "rendering".
    ///
    /// - `init`: the seed value of the folding
    /// - `coeff`: if a coefficient can be displayed, do something to the accumulator
    /// - `var`: if a variable can be displayed, do something to the accumulator
    /// - `sep`: do something to the accumulator to add a seperator
    pub fn fold_display_parts<O, C, V, S>(&self, init: O, coeff: C, var: V, sep: S) -> O
    where
        C: Fn(&mut O, R::Element),
        V: Fn(&mut O, Option<usize>),
        S: Fn(&mut O),
    {
        let parts = DisplayPart::get_parts(self);
        let len = parts.len();

        parts
            .into_iter()
            .enumerate()
            .fold(init, |mut acc, (i, part)| {
                if let Some(c) = part.coefficient {
                    coeff(&mut acc, c);
                }

                if let Some(exponent) = part.variable {
                    var(&mut acc, exponent);
                }

                if i < len - 1 {
                    sep(&mut acc);
                }

                acc
            })
    }

    /// Map components of a displayable [Polynomial] to other types.
    ///
    /// This uses [DisplayPart]s for "rendering".
    ///
    /// - `coeff`: if a coefficient can be displayed, map it to something
    /// - `var`: if a variable can be displayed, map it to something
    /// - `sep`: separator between the components
    pub fn map_display_parts<C, CF, V, VF, S, SF>(
        &self,
        coeff: CF,
        var: VF,
        sep: SF,
    ) -> impl IntoIterator<Item = (Option<C>, Option<V>, Option<S>)>
    where
        CF: Fn(R::Element) -> C,
        VF: Fn(Option<usize>) -> V,
        SF: Fn() -> S,
    {
        let parts = DisplayPart::get_parts(self);
        let len = parts.len();

        parts.into_iter().enumerate().map(move |(i, part)| {
            let c = part.coefficient.map(&coeff);
            let v = part.variable.map(&var);
            let s = (i < len - 1).then(&sep);
            (c, v, s)
        })
    }
}

impl<R> fmt::Display for Polynomial<R>
where
    R: DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self.fold_display_parts(
            String::new(),
            |s, coeff| {
                s.push_str(&coeff.to_string());
            },
            |s, exp| {
                s.push('x');
                if let Some(exp) = exp {
                    s.push('^');
                    s.push_str(&exp.to_string());
                }
            },
            |s| {
                s.push_str(" + ");
            },
        );

        write!(f, "{string}")
    }
}

impl<R> fmt::Debug for Polynomial<R>
where
    R: Ring + fmt::Debug,
    R::Element: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Polynomial")
            .field("ring", &self.ring)
            .field("elems", &self.elems)
            .finish()
    }
}
