use polymoly::{Polynomial, ZMod};

fn main() {
    let mut p: Polynomial<ZMod<5>> = Polynomial::with_capacity(3);
    p.add_elem(3, 2);
    p.add_elem(1, 1);
    p.add_elem(4, 0);
    let mut q: Polynomial<ZMod<5>> = Polynomial::with_capacity(3);
    q.add_elem(1, 1);
    q.add_elem(2, 0);

    println!("{:?}", p * q);
}
