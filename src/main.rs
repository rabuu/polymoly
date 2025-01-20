use polymoly::structures::{PolyRing, R, Z};
use polymoly::Poly;

fn main() {
    let f: Poly<Z> = Poly::new([1, 1, 0, 1]);
    let g: Poly<Z> = Poly::new([-1, 1]);
    println!("{:?}", f.clone() + g.clone());
    println!("{}", f.clone() + g.clone());

    let mega_poly: Poly<PolyRing<Z>> = Poly::new([f, g]);

    println!("{:?}", mega_poly);
}
