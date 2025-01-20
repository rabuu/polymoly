use polymoly::structures::R;
use polymoly::Poly;

fn main() {
    let f: Poly<R> = Poly::new([1.0, 1.0, 0.0, 1.0]);
    let g: Poly<R> = Poly::new([-1.0, 1.0]);
    println!("{:#?}", f + g);
}
