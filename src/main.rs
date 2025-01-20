use polymoly::{Polynomial, ZMod};

fn main() {
    let p: Polynomial<ZMod<5>> = Polynomial::constant(7);
    let q: Polynomial<ZMod<5>> = Polynomial::constant(2);

    println!("{:?}", p == q);
}
