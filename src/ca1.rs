use std::{error::Error, num::ParseIntError, str::FromStr};

use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg64;

use crate::options::HasOptions;
use crate::time_space;
use crate::{Apply, Options, State, StateIter};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[clap(long, value_parser = Rule::from_arg)]
    rule: Option<Rule>,

    #[clap(flatten)]
    opts: Options,
}

impl HasOptions for Args {
    fn options(&self) -> &Options {
        &self.opts
    }
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    println!("{:?}", args);
    let mut rng = Pcg64::seed_from_u64(args.seed().value());

    let (theme, colors) = args.theme().pick(&mut rng)?;

    let rule = args.rule.unwrap_or_else(|| Rule::new(rng.gen()));

    println!("seed: {}, theme: {}, rule: {}", args.seed(), theme, rule);

    let mut state = State::with_size(args.cols() as usize);
    state.set(state.len() as i32 / 2, 1);

    let iter = StateIter::new(rule, state);
    time_space::render_to(
        args.dest(),
        args.cols(),
        args.rows(),
        args.cell_size(),
        &colors,
        iter,
    )?;

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rule {
    v: u8,
}

impl Rule {
    fn new(v: u8) -> Self {
        Self { v }
    }

    fn from_arg(s: &str) -> Result<Self, String> {
        Self::from_str(s).map_err(|e| e.to_string())
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rule::new(match s.strip_prefix("0x") {
            Some(s) => u8::from_str_radix(s, 16)?,
            None => s.parse()?,
        }))
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:x}", self.v)
    }
}

impl Apply for Rule {
    fn apply(&self, a: u8, b: u8, c: u8) -> u8 {
        let p = (a & 1) << 2 | (b & 1) << 1 | (c & 1);
        (self.v >> p) & 1
    }
}
