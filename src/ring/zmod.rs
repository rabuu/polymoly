//! The ring of integers modulo n `Z/nZ`

use super::{Field, Ring};

/// The ring of integers modulo n `Z/nZ`
#[derive(Debug, Clone, Copy)]
pub struct ZMod<const N: usize>;

impl<const N: usize> Ring for ZMod<N> {
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
        elem.rem_euclid(N as isize)
    }
}

macro_rules! impl_field_for_zmod {
    ($n:expr) => {
        impl Field for ZMod<$n> {
            fn inv(&self, elem: Self::Element) -> Option<Self::Element> {
                let (_, s, _) = crate::euclid::extended_euclidean_int(elem, $n)?;
                Some(s.rem_euclid($n))
            }
        }
    };
}

/// The primes which are automatically implemented (2 - 127)
pub const IMPLEMENTED_PRIMES: [usize; 32] = [
    2, 3, 5, 7, 9, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127,
];

impl_field_for_zmod!(2);
impl_field_for_zmod!(3);
impl_field_for_zmod!(5);
impl_field_for_zmod!(7);
impl_field_for_zmod!(9);
impl_field_for_zmod!(11);
impl_field_for_zmod!(13);
impl_field_for_zmod!(17);
impl_field_for_zmod!(19);
impl_field_for_zmod!(23);
impl_field_for_zmod!(29);
impl_field_for_zmod!(31);
impl_field_for_zmod!(37);
impl_field_for_zmod!(41);
impl_field_for_zmod!(43);
impl_field_for_zmod!(47);
impl_field_for_zmod!(53);
impl_field_for_zmod!(59);
impl_field_for_zmod!(61);
impl_field_for_zmod!(67);
impl_field_for_zmod!(71);
impl_field_for_zmod!(73);
impl_field_for_zmod!(79);
impl_field_for_zmod!(83);
impl_field_for_zmod!(89);
impl_field_for_zmod!(97);
impl_field_for_zmod!(101);
impl_field_for_zmod!(103);
impl_field_for_zmod!(107);
impl_field_for_zmod!(109);
impl_field_for_zmod!(113);
impl_field_for_zmod!(127);
