use polymoly::{Polynomial, ZMod};

fn main() {
    let mut p: Polynomial<ZMod<5>> = Polynomial::new();
    p.add_elem(7, 3);
    p.add_elem(1, 0);
    println!("p = {p:?}");

    let mut q: Polynomial<ZMod<5>> = Polynomial::new();
    q.add_elem(3, 3);
    println!("q = {q:?}");

    println!("{:#?}", q + p + Polynomial::constant(19));
}
