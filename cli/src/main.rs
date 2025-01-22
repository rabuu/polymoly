use std::fmt;

use clap::{error::ErrorKind, ArgGroup, CommandFactory, Parser, Subcommand};

use polymoly::parse::{DisplayRing, ParsableRing};
use polymoly::{Poly, ZModN, ZModP, R, Z};

#[derive(Parser)]
#[command(version, propagate_version = true, about = None, long_about = None)]
#[command(group(ArgGroup::new("ring").multiple(false)))]
struct CliArgs {
    #[command(subcommand)]
    operation: Operation,

    /// Interpret input as polynomial over real numbers
    #[arg(short = 'R', long, group = "ring")]
    real: bool,

    /// Interpret input as polynomial over integers
    #[arg(short = 'Z', long, group = "ring")]
    integer: bool,

    /// Interpret input as polynomial over integers modulo n
    #[arg(long, group = "ring")]
    zmodn: Option<usize>,

    /// Interpret input as polynomial over integers modulo p (where p must be prime)
    #[arg(long, group = "ring", group = "prime")]
    zmodp: Option<usize>,

    /// Don't check if p is prime
    #[arg(long, requires = "prime")]
    dont_check_primes: bool,
}

#[derive(Debug, Subcommand)]
enum Operation {
    /// Add two polynomials
    Add { lhs: String, rhs: String },

    /// Multiply two polynomials
    Mul { lhs: String, rhs: String },
}

fn main() {
    let cli = CliArgs::parse();

    match (cli.real, cli.integer, cli.zmodn, cli.zmodp) {
        (false, true, None, None) => run(Z, cli.operation),
        (false, false, Some(n), None) => run(ZModN::new(n), cli.operation),
        (false, false, None, Some(p)) => {
            if cli.dont_check_primes {
                run(ZModP::new(p), cli.operation)
            } else if let Some(zmodp) = ZModP::checked_new(p) {
                run(zmodp, cli.operation)
            } else {
                let mut cmd = CliArgs::command();
                cmd.error(ErrorKind::InvalidValue, "Argument of ZModP must be prime")
                    .exit();
            }
        }
        _ => run(R, cli.operation),
    }
}

fn run<R>(ring: R, operation: Operation)
where
    R: ParsableRing + DisplayRing,
    R::Element: Clone + PartialEq + fmt::Display,
{
    match operation {
        Operation::Add { lhs, rhs } => {
            let Some(lhs) = Poly::parse(ring, &lhs) else {
                let mut cmd = CliArgs::command();
                cmd.error(ErrorKind::InvalidValue, "LHS is no valid polynomial")
                    .exit();
            };

            let Some(rhs) = Poly::parse(ring, &rhs) else {
                let mut cmd = CliArgs::command();
                cmd.error(ErrorKind::InvalidValue, "RHS is no valid polynomial")
                    .exit();
            };

            println!("{}", lhs + rhs);
        }
        Operation::Mul { lhs, rhs } => {
            let Some(lhs) = Poly::parse(ring, &lhs) else {
                let mut cmd = CliArgs::command();
                cmd.error(ErrorKind::InvalidValue, "LHS is no valid polynomial")
                    .exit();
            };

            let Some(rhs) = Poly::parse(ring, &rhs) else {
                let mut cmd = CliArgs::command();
                cmd.error(ErrorKind::InvalidValue, "RHS is no valid polynomial")
                    .exit();
            };

            println!("{}", lhs * rhs);
        }
    }
}
