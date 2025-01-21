use polymoly::{ZModN, R, Z};
use polymoly::Poly;

fn main() {
    let g = Poly::parse(R, "0.5x+-3x^2+5x^1+1x^0").unwrap();
    let h = Poly::parse(Z, "4x+-3x^2+5x^1+1x^0").unwrap();
    let l = Poly::parse(ZModN::new(7), "4x+-3x^2+5x^1+1x^0").unwrap();
    println!("{g}\n{h}\n{l}");
    println!("{g:?}\n{h:?}\n{l:?}");
}
