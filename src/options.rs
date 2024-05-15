use std::error::Error;

use clap::{Parser, Subcommand};
use rand::{RngCore, SeedableRng};
use rand_pcg::Pcg64;

use crate::{ca1, ca4, Color, Seed, ThemeRef};

#[derive(Parser, Debug)]
pub struct Options {
    #[clap(long, default_value_t = 400)]
    rows: i32,

    #[clap(long, default_value_t = 100)]
    cols: i32,

    #[clap(long, default_value_t = 6)]
    cell_size: i32,

    #[clap(long, default_value_t = ThemeRef::from_path("themes.bin"), value_parser=ThemeRef::from_arg)]
    theme: ThemeRef,

    #[clap(long, default_value_t = Default::default(), value_parser = Seed::from_arg)]
    seed: Seed,

    #[clap(long, default_value = "ca.png")]
    dest: String,
}

impl Options {
    pub fn seed(&self) -> &Seed {
        &self.seed
    }

    pub fn theme(&self) -> &ThemeRef {
        &self.theme
    }

    pub fn dest(&self) -> &str {
        &self.dest
    }

    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }

    pub fn cell_size(&self) -> i32 {
        self.cell_size
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(name = "1")]
    Ca1(ca1::Args),
    #[clap(name = "4")]
    Ca4(ca4::Args),
}

impl Command {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Ca1(args) => ca1::run(args),
            Command::Ca4(args) => ca4::run(args),
        }
    }
}

pub trait HasOptions {
    fn options(&self) -> &Options;

    fn seed(&self) -> &Seed {
        self.options().seed()
    }

    fn theme(&self) -> &ThemeRef {
        self.options().theme()
    }

    fn dest(&self) -> &str {
        self.options().dest()
    }

    fn cols(&self) -> i32 {
        self.options().cols()
    }

    fn rows(&self) -> i32 {
        self.options().rows()
    }

    fn cell_size(&self) -> i32 {
        self.options().cell_size()
    }

    fn generate<F, T>(&self, f: F) -> Result<(usize, Vec<Color>, T), Box<dyn Error>>
    where
        F: Fn(&mut dyn RngCore) -> T,
    {
        let mut rng = Pcg64::seed_from_u64(self.seed().value());
        let (theme, colors) = self.theme().pick(&mut rng)?;
        Ok((theme, colors, f(&mut rng)))
    }
}
