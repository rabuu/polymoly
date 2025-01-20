use polymoly::Polynomial;
use polymoly::structures::R;

fn main() {
    let f: Polynomial<R> = Polynomial::new([1.0, 1.0, 0.0, 1.0]);
    let g: Polynomial<R> = Polynomial::new([-1.0, 1.0]);
    println!("{:?}", f.polynomial_division(g));
}
