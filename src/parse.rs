use crate::{Poly, Ring, ZMod, R, Z};

pub trait ParsableRing: Ring {
    fn parse_elem(&self, input: &str) -> Option<Self::Element>;

    fn parse_poly(&self, input: &str) -> Option<Poly<Self>>
    where
        Self: Sized,
        Self::Element: Clone + PartialEq,
    {
        let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        let mut poly: Poly<Self> = Poly::zero(*self);
        for summand in input.split('+') {
            if let Some((coeff, pot)) = summand.split_once('x') {
                let pot = if let Some(pot) = pot.strip_prefix('^') {
                    pot.parse().ok()
                } else {
                    (pot.is_empty()).then_some(1)
                };

                if let (Some(coeff), Some(pot)) = (self.parse_elem(coeff), pot) {
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

impl ParsableRing for R {
    fn parse_elem(&self, input: &str) -> Option<Self::Element> {
        input.parse().ok()
    }
}

impl ParsableRing for Z {
    fn parse_elem(&self, input: &str) -> Option<Self::Element> {
        input.parse().ok()
    }
}

impl<const N: usize> ParsableRing for ZMod<N> {
    fn parse_elem(&self, input: &str) -> Option<Self::Element> {
        input.parse().ok().map(|e| self.id(e))
    }
}
