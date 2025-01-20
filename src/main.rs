use polymoly::Polynomial;
use polymoly::structures::R;

fn main() {
    let mut f: Polynomial<R> = Polynomial::new();
    f.add_elem(1.0, 3);
    f.add_elem(1.0, 1);
    f.add_elem(1.0, 0);

    let mut g: Polynomial<R> = Polynomial::new();
    g.add_elem(1.0, 1);
    g.add_elem(-1.0, 0);

    println!("{:?}", f.polynomial_division(g));
}
