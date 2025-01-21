use polymoly::structures::{ZMod, R, Z};
use polymoly::Poly;

fn main() {
    let g: Poly<R> = "0.5x+-3x^2+5x^1+1x^0".parse().unwrap();
    let h: Poly<Z> = "4x+-3x^2+5x^1+1x^0".parse().unwrap();
    let l: Poly<ZMod<7>> = "4x+3x^2+5x^1+1x^0".parse().unwrap();
    println!("{g}\n{h}\n{l}");
}
