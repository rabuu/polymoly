use std::fmt;

use clap::ArgGroup;
use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};

use polymoly::poly::{display::DisplayRing, parse::ParsableRing};
use polymoly::{Field, Poly, ZModN, ZModP, R, Z};

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
        .fold(Poly::zero(ring), |acc, p| acc + p);

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
        .fold(Poly::zero(ring), |acc, p| acc * p);

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
