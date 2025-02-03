//! # PolyMoly
//! A library for polynomials over various rings.
//!
//! ## Example usage
//! ```
//! # use polymoly::ring::IntegersModuloN;
//! # use polymoly::polynomial::Polynomial;
//! let z_mod_5 = IntegersModuloN::new(5);
//!
//! let f = Polynomial::new(z_mod_5, vec![1, 0, 3]);
//! let g = Polynomial::parse(z_mod_5, "3x^2 + x + 7").unwrap();
//!
//! assert_eq!(format!("{f}"), String::from("3x^2 + 1"));
//! assert_eq!(f + g, Polynomial::new(z_mod_5, vec![3, 1, 1]));
//! ```

pub mod euclid;
pub mod polynomial;
pub mod ring;
