use polymoly::{Polynomial, Z};

fn main() {
    let mut p: Polynomial<Z> = Polynomial::new();
    p.add_elem(7, 3);
    p.add_elem(1, 0);
    println!("p = {p:?}");

    let mut q: Polynomial<Z> = Polynomial::new();
    q.add_elem(-7, 3);
    println!("q = {q:?}");

    println!("{:#?}", q + p + Polynomial::constant(19));
}
