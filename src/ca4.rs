use std::{error::Error, num::ParseIntError, str::FromStr};

use crate::{Apply, Options};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[clap(long, value_parser = Rule::from_arg)]
    rule: Option<Rule>,

    #[clap(flatten)]
    opts: Options,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    println!("ca4 {:?}", args);
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rule {
    v: u64,
}

impl Rule {
    fn new(v: u64) -> Self {
        Self { v }
    }

    fn apply(&self, p: u8) -> u8 {
        ((self.v >> p) & 3) as u8
    }

    fn from_arg(s: &str) -> Result<Self, String> {
        Self::from_str(s).map_err(|e| e.to_string())
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rule::new(match s.strip_prefix("0x") {
            Some(s) => u64::from_str_radix(s, 16)?,
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
        let p = (a & 3) << 4 | (b & 3) << 2 | (c & 3);
        self.apply(p)
    }
}
