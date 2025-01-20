use crate::euclid::extended_euclidean_int;

use super::Ring;
use super::{ZMod, R};

pub trait Field: Ring {
    fn inv(elem: Self::T) -> Self::T;

    fn div(lhs: Self::T, rhs: Self::T) -> Self::T {
        Self::mul(lhs, Self::inv(rhs))
    }
}

impl Field for R {
    fn inv(elem: f64) -> f64 {
        1.0 / elem
    }
}

macro_rules! impl_field_for_zmod {
    ($n:expr) => {
        impl Field for ZMod<$n> {
            fn inv(elem: usize) -> usize {
                inv_mod_n(elem, $n).expect("division by zero")
            }
        }
    };
}

fn inv_mod_n(elem: usize, n: usize) -> Option<usize> {
    let elem = elem % n;
    let (_, s, _) = extended_euclidean_int(elem as isize, n as isize)?;
    Some(s.rem_euclid(n as isize) as usize)
}

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
