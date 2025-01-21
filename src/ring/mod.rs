pub mod integer;
pub mod poly_ring;
pub mod real;
pub mod zmod;

/// A commutative ring
pub trait Ring {
    type Element;

    fn zero() -> Self::Element;
    fn one() -> Self::Element;

    fn add(lhs: Self::Element, rhs: Self::Element) -> Self::Element;
    fn neg(elem: Self::Element) -> Self::Element;
    fn mul(lhs: Self::Element, rhs: Self::Element) -> Self::Element;

    fn sub(lhs: Self::Element, rhs: Self::Element) -> Self::Element {
        Self::add(lhs, Self::neg(rhs))
    }

    fn id(elem: Self::Element) -> Self::Element {
        elem
    }
}

pub trait Field: Ring {
    fn inv(elem: Self::Element) -> Option<Self::Element>;

    fn div(lhs: Self::Element, rhs: Self::Element) -> Option<Self::Element> {
        Self::inv(rhs).map(|inv| Self::mul(lhs, inv))
    }
}
