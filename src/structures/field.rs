use crate::euclid::extended_euclidean_int;

use super::Ring;
use super::{ZMod, R};

pub trait Field: Ring {
    fn inv(elem: Self::Element) -> Option<Self::Element>;

    fn div(lhs: Self::Element, rhs: Self::Element) -> Option<Self::Element> {
        Self::inv(rhs).map(|inv| Self::mul(lhs, inv))
    }
}

impl Field for R {
    fn inv(elem: Self::Element) -> Option<Self::Element> {
        (elem != 0.0).then_some(1.0 / elem)
    }
}

fn inv_mod_n(elem: isize, n: isize) -> Option<isize> {
    let elem = elem.rem_euclid(n);
    let (_, s, _) = extended_euclidean_int(elem, n)?;
    Some(s.rem_euclid(n))
}

macro_rules! impl_field_for_zmod {
    ($n:expr) => {
        impl Field for ZMod<$n> {
            fn inv(elem: Self::Element) -> Option<Self::Element> {
                inv_mod_n(elem, $n)
            }
        }
    };
}

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
