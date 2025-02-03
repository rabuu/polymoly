use std::fmt;

use clap::ArgGroup;
use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};

use polymoly::polynomial::display::DisplayRing;
use polymoly::polynomial::parse::ParsableRing;
use polymoly::polynomial::Polynomial;
use polymoly::ring::{Field, Integers, IntegersModuloN, IntegersModuloP, PolynomialRing, Reals};

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
    reals: bool,

    /// Interpret polynomials over integers
    #[arg(short = 'Z', long)]
    integers: bool,

    /// Interpret polynomials over integers modulo n
    #[arg(short = 'M', long, value_name = "N")]
    modulo: Option<usize>,
}

impl RingArg {
    fn run<R, I, M>(&self, reals: R, integers: I, modulo: M)
    where
        R: Fn(Reals),
        I: Fn(Integers),
        M: Fn(IntegersModuloN),
    {
        match (self.reals, self.integers, self.modulo) {
            (false, true, None) => integers(Integers),
            (false, false, Some(n)) => modulo(IntegersModuloN::new(n)),
            _ => reals(Reals),
        }
    }
}

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("field").multiple(false)))]
struct FieldArg {
    /// Interpret polynomials over real numbers
    #[arg(short = 'R', long, group = "field")]
    reals: bool,

    /// Interpret polynomials over integers modulo p (where p is prime)
    #[arg(
        short = 'M',
        long,
        value_name = "P",
        group = "field",
        group = "prime check"
    )]
    modulo: Option<usize>,

    /// Don't check if p is actually a prime number
    #[arg(long, requires = "prime check")]
    disable_prime_check: bool,
}

impl FieldArg {
    fn run<R, M>(&self, reals: R, modulo: M)
    where
        R: Fn(Reals),
        M: Fn(IntegersModuloP),
    {
        match (self.reals, self.modulo) {
            (false, Some(p)) => {
                if self.disable_prime_check {
                    modulo(IntegersModuloP::new_unchecked(p))
                } else if let Some(p) = IntegersModuloP::new(p) {
                    modulo(p)
                } else {
                    let mut cmd = CliArgs::command();
                    cmd.error(ErrorKind::InvalidValue, "Argument p must be prime")
                        .exit();
                }
            }
            _ => reals(Reals),
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
    poly_reals: bool,

    /// Interpret polynomials over integers modulo p (where p is prime)
    #[arg(
        short = 'M',
        long,
        value_name = "P",
        group = "euclidean ring",
        group = "prime check"
    )]
    poly_modulo: Option<usize>,

    /// Don't check if p is actually a prime number
    #[arg(long, requires = "prime check")]
    disable_prime_check: bool,
}

impl EuclideanRingArg {
    fn run<I, R, M>(&self, integers: I, reals: R, modulo: M)
    where
        I: Fn(Integers),
        R: Fn(Reals),
        M: Fn(IntegersModuloP),
    {
        match (self.integers, self.poly_reals, self.poly_modulo) {
            (true, false, None) => integers(Integers),
            (false, true, None) => reals(Reals),
            (false, false, Some(p)) => {
                if self.disable_prime_check {
                    modulo(IntegersModuloP::new_unchecked(p))
                } else if let Some(p) = IntegersModuloP::new(p) {
                    modulo(p)
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
    R::Element: fmt::Display,
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
    R::Element: fmt::Display,
{
    let lhs = parse_polynomial(ring, lhs);
    let rhs = parse_polynomial(ring, rhs);

    println!("{}", lhs - rhs);
}

fn mul<R>(ring: R, polynomials: &[String])
where
    R: ParsableRing + DisplayRing,
    R::Element: fmt::Display,
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
    F::Element: fmt::Display,
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

    println!("{gcd}\nWITH s = {s} AND t = {t}");
}

fn gcd_poly<F>(field: F, lhs: &str, rhs: &str)
where
    F: Field + ParsableRing + DisplayRing,
    F::Element: fmt::Display,
{
    let lhs = parse_polynomial(field, lhs);
    let rhs = parse_polynomial(field, rhs);

    let poly_ring = PolynomialRing::new(field);
    let Some((gcd, s, t)) = polymoly::euclid::extended_euclidean(poly_ring, lhs, rhs) else {
        let mut cmd = CliArgs::command();
        cmd.error(ErrorKind::InvalidValue, "One side must be non-zero")
            .exit();
    };

    println!("{gcd}\nWITH s = {s} AND t = {t}");
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

fn parse_polynomial<R: ParsableRing>(ring: R, input: &str) -> Polynomial<R> {
    if let Some(poly) = Polynomial::parse(ring, input) {
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
