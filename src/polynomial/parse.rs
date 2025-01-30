//! Parsing of polynomials

use crate::ring::{Integers, IntegersModuloAny, Reals, Ring};

use super::Polynomial;

/// A ring where polynomials can be parsed
pub trait ParsableRing: Ring {
    fn parse_elem(&self, input: &str) -> Option<Self::Element>;

    fn parse_poly(&self, input: &str) -> Option<Polynomial<Self>> {
        let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        let mut poly: Polynomial<Self> = Polynomial::zero(*self);
        for summand in input.split('+') {
            if let Some((coeff, pot)) = summand.split_once('x') {
                let pot = if let Some(pot) = pot.strip_prefix('^') {
                    pot.parse().ok()
                } else {
                    (pot.is_empty()).then_some(1)
                };

                let coeff = if coeff.is_empty() {
                    Some(self.one())
                } else {
                    self.parse_elem(coeff)
                };

                if let (Some(coeff), Some(pot)) = (coeff, pot) {
                    poly.add_elem(coeff, pot);
                    continue;
                }
            } else if let Some(constant) = self.parse_elem(summand) {
                poly.add_elem(constant, 0);
                continue;
            }

            return None;
        }

        Some(poly)
    }
}

impl ParsableRing for Reals {
    fn parse_elem(&self, input: &str) -> Option<Self::Element> {
        input.parse().ok()
    }
}

impl ParsableRing for Integers {
    fn parse_elem(&self, input: &str) -> Option<Self::Element> {
        input.parse().ok()
    }
}

impl<T: IntegersModuloAny> ParsableRing for T {
    fn parse_elem(&self, input: &str) -> Option<Self::Element> {
        input.parse().ok().map(|e| self.id(e))
    }
}
