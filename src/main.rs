use polymoly::euclid::extended_euclidean;
use polymoly::structures::{PolyRing, ZMod, Z};
use polymoly::Poly;

fn main() {
    let g: Poly<ZMod<5>> = Poly::new([3, 3, 0, 0, 2, 1]);
    let h: Poly<ZMod<5>> = Poly::new([2, 3, 2, 2, 1]);
    let (gcd, s, t) = extended_euclidean::<PolyRing<ZMod<5>>>(g, h).unwrap();
    println!("GCD: {gcd}, s: {s}, t: {t}");

    let (gcd, s, t) = extended_euclidean::<Z>(-48, 30).unwrap();
    println!("GCD: {gcd}, s: {s}, t: {t}");
}
