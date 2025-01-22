use std::fmt;

use clap::ArgGroup;
use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};

use polymoly::poly::{display::DisplayRing, parse::ParsableRing};
use polymoly::{Field, Poly, PolyRing, ZModN, ZModP, R, Z};

#[derive(Parser)]
#[command(version, propagate_version = true, about = None, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Debug, Subcommand)]
enum Operation {
    /// Add a number of polynomials
    Add {
        #[command(flatten)]
        ring: RingArg,

        /// At least two polynomials
        #[arg(num_args = 2..)]
        poly: Vec<String>,
    },

    /// Subtract one polynomial from another
    Sub {
        #[command(flatten)]
        ring: RingArg,

        /// Left-hand side polynomial
        lhs: String,

        /// Right-hand side polynomial
        rhs: String,
    },

    /// Multiply a number of polynomials
    Mul {
        #[command(flatten)]
        ring: RingArg,

        /// At least two polynomials
        #[arg(num_args = 2..)]
        poly: Vec<String>,
    },

    /// Polynomial division of two polynomials
    Div {
        #[command(flatten)]
        field: FieldArg,

        /// Left-hand side polynomial
        lhs: String,

        /// Right-hand side polynomial
        rhs: String,
    },

    /// Greatest common divisor (using EEA) in a euclidean ring
    Gcd {
        #[command(flatten)]
        ring: EuclideanRingArg,

        /// Left-hand side
        lhs: String,

        /// Right-hand side
        rhs: String,
    },
}

#[derive(Debug, Args)]
#[group(multiple = false)]
struct RingArg {
    /// Interpret polynomials over real numbers
    #[arg(short = 'R', long)]
    real: bool,

    /// Interpret polynomials over integers
    #[arg(short = 'Z', long)]
    integer: bool,

    /// Interpret polynomials over integers modulo n
    #[arg(short = 'N', long, value_name = "N")]
    zmod: Option<usize>,
}

impl RingArg {
    fn run<Real, Int, Mod>(&self, real: Real, integer: Int, zmodn: Mod)
    where
        Real: Fn(R),
        Int: Fn(Z),
        Mod: Fn(ZModN),
    {
        match (self.real, self.integer, self.zmod) {
            (false, true, None) => integer(Z),
            (false, false, Some(n)) => zmodn(ZModN::new(n)),
            _ => real(R),
        }
    }
}

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("field").multiple(false)))]
struct FieldArg {
    /// Interpret polynomials over real numbers
    #[arg(short = 'R', long, group = "field")]
    real: bool,

    /// Interpret polynomials over integers modulo p (where p is prime)
    #[arg(
        short = 'P',
        long,
        value_name = "P",
        group = "field",
        group = "prime check"
    )]
    zmod: Option<usize>,

    /// Don't check if p is actually a prime number
    #[arg(long, requires = "prime check")]
    disable_prime_check: bool,
}

impl FieldArg {
    fn run<Real, Mod>(&self, real: Real, zmodp: Mod)
    where
        Real: Fn(R),
        Mod: Fn(ZModP),
    {
        match (self.real, self.zmod) {
            (false, Some(p)) => {
                if self.disable_prime_check {
                    zmodp(ZModP::new(p))
                } else if let Some(p) = ZModP::checked_new(p) {
                    zmodp(p)
                } else {
                    let mut cmd = CliArgs::command();
                    cmd.error(ErrorKind::InvalidValue, "Argument p must be prime")
                        .exit();
                }
            }
            _ => real(R),
        }
    }
}

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("euclidean ring").required(true).multiple(false)))]
struct EuclideanRingArg {
    /// Interpret as simple integers
    #[arg(short = 'Z', long, group = "euclidean ring")]
    integers: bool,

    /// Interpret polynomials over real numbers
    #[arg(short = 'R', long, group = "euclidean ring")]
    poly_real: bool,

    /// Interpret polynomials over integers modulo p (where p is prime)
    #[arg(
        short = 'P',
        long,
        value_name = "P",
        group = "euclidean ring",
        group = "prime check"
    )]
    poly_zmod: Option<usize>,

    /// Don't check if p is actually a prime number
    #[arg(long, requires = "prime check")]
    disable_prime_check: bool,
}

impl EuclideanRingArg {
    fn run<Int, Real, Mod>(&self, int: Int, real: Real, zmodp: Mod)
    where
        Int: Fn(Z),
        Real: Fn(R),
        Mod: Fn(ZModP),
    {
        match (self.integers, self.poly_real, self.poly_zmod) {
            (true, false, None) => int(Z),
            (false, true, None) => real(R),
            (false, false, Some(p)) => {
                if self.disable_prime_check {
                    zmodp(ZModP::new(p))
                } else if let Some(p) = ZModP::checked_new(p) {
                    zmodp(p)
                } else {
                    let mut cmd = CliArgs::command();
                    cmd.error(ErrorKind::InvalidValue, "Argument p must be prime")
                        .exit();
                }
            }
            _ => unreachable!("clap: required and no multiple"),
        }
    }
}

fn main() {
    let cli = CliArgs::parse();

    match cli.operation {
        Operation::Add { ring, poly } => {
            ring.run(|r| add(r, &poly), |z| add(z, &poly), |n| add(n, &poly))
        }
        Operation::Sub { ring, lhs, rhs } => ring.run(
            |r| sub(r, &lhs, &rhs),
            |z| sub(z, &lhs, &rhs),
            |n| sub(n, &lhs, &rhs),
        ),
        Operation::Mul { ring, poly } => {
            ring.run(|r| mul(r, &poly), |z| mul(z, &poly), |n| mul(n, &poly))
        }
        Operation::Div { field, lhs, rhs } => {
            field.run(|r| div(r, &lhs, &rhs), |p| div(p, &lhs, &rhs))
        }
        Operation::Gcd { ring, lhs, rhs } => ring.run(
            |_| gcd_int(&lhs, &rhs),
            |r| gcd_poly(r, &lhs, &rhs),
            |p| gcd_poly(p, &lhs, &rhs),
        ),
    }
}

fn add<R>(ring: R, polynomials: &[String])
where
    R: ParsableRing + DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    let result = polynomials
        .iter()
        .map(|s| parse_polynomial(ring, s))
        .reduce(|acc, p| acc + p)
        .expect("at least two polynomials");

    println!("{result}");
}

fn sub<R>(ring: R, lhs: &str, rhs: &str)
where
    R: ParsableRing + DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    let lhs = parse_polynomial(ring, lhs);
    let rhs = parse_polynomial(ring, rhs);

    println!("{}", lhs - rhs);
}

fn mul<R>(ring: R, polynomials: &[String])
where
    R: ParsableRing + DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    let result = polynomials
        .iter()
        .map(|s| parse_polynomial(ring, s))
        .reduce(|acc, p| acc * p)
        .expect("at least two polynomials");

    println!("{result}");
}

fn div<F>(field: F, lhs: &str, rhs: &str)
where
    F: Field + ParsableRing + DisplayRing,
    F::Element: Clone + PartialEq + fmt::Display,
{
    let lhs = parse_polynomial(field, lhs);
    let rhs = parse_polynomial(field, rhs);
    let Some((q, r)) = lhs.clone().polynomial_division(rhs.clone()) else {
        let mut cmd = CliArgs::command();
        cmd.error(ErrorKind::InvalidValue, "Right-hand side must not be zero")
            .exit();
    };

    println!("{q}");
    if !r.is_zero() {
        println!("REM {r}");
    }
}

fn gcd_int(lhs: &str, rhs: &str) {
    let lhs = parse_int(lhs);
    let rhs = parse_int(rhs);

    let Some((gcd, s, t)) = polymoly::euclid::extended_euclidean_int(lhs, rhs) else {
        let mut cmd = CliArgs::command();
        cmd.error(ErrorKind::InvalidValue, "One side must be non-zero")
            .exit();
    };

    println!("{gcd} with s = {s} and t = {t}");
}

fn gcd_poly<F>(field: F, lhs: &str, rhs: &str)
where
    F: Field + ParsableRing + DisplayRing,
    F::Element: Clone + PartialEq + fmt::Display,
{
    let lhs = parse_polynomial(field, lhs);
    let rhs = parse_polynomial(field, rhs);

    let poly_ring = PolyRing::new(field);
    let Some((gcd, s, t)) = polymoly::euclid::extended_euclidean(poly_ring, lhs, rhs) else {
        let mut cmd = CliArgs::command();
        cmd.error(ErrorKind::InvalidValue, "One side must be non-zero")
            .exit();
    };

    println!("{gcd} with s = {s} and t = {t}");
}

fn parse_int(input: &str) -> isize {
    if let Ok(int) = input.parse() {
        int
    } else {
        let mut cmd = CliArgs::command();
        cmd.error(
            ErrorKind::InvalidValue,
            format!("`{input}` cannot be parsed as integer"),
        )
        .exit();
    }
}

fn parse_polynomial<R>(ring: R, input: &str) -> Poly<R>
where
    R: ParsableRing,
    R::Element: Clone + PartialEq,
{
    if let Some(poly) = Poly::parse(ring, input) {
        poly
    } else {
        let mut cmd = CliArgs::command();
        cmd.error(
            ErrorKind::InvalidValue,
            format!("`{input}` cannot be parsed as polynomial in `{ring:?}`"),
        )
        .exit();
    }
}
