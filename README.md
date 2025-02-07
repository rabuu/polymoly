# PolyMoly
A polynomial calculator with support for various rings.

The project consists of three parts: The Rust library, a command line tool and a webapp.

## Library
The Rust library crate implements the core logic of algebraic rings and calculating with polynomials.

Documentation is available [here](https://rbuurman.de/projekte/rustdoc/polymoly).

### Example usage
```rust
use polymoly::ring::IntegersModuloN;
use polymoly::polynomial::Polynomial;

let z_mod_5 = IntegersModuloN::new(5);

let f = Polynomial::new(z_mod_5, vec![1, 0, 3]);
let g = Polynomial::parse(z_mod_5, "3x^2 + x + 7").unwrap();

assert_eq!(format!("{f}"), String::from("3x^2 + 1"));
assert_eq!(f + g, Polynomial::new(z_mod_5, vec![3, 1, 1]));
```

## Command Line Tool
The `polymoly` binary provides a simple calculator interface as CLI tool.

See `polymoly --help` for all operations and `polymoly <OP> --help` their options.

### Example usage
```
$ polymoly add '3x + 4x^2' '0.5 + x^2'
4x^2 + 3x + 0.5

$ polymoly mul --integers '3x^2' '2' 'x'
6x^3

$ polymoly div --modulo=2 'x^4 + 1' 'x^3 + x + 1'
x
REM x^2 + x + 1

$ polymoly gcd --poly-reals 'x^2 + 2x + 1' 'x^2 + x'
x + 1
WITH s = 1 AND t = -1
```

## Webapp
The webapp runs [here](https://rbuurman.de/projekte/polymoly). It has basically the same functionality as the CLI tool.
