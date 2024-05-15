pub mod ca1;
pub mod ca4;
mod color;
mod options;
mod seed;
mod state;
mod state_iter;
mod themes;
pub mod time_space;

pub use color::Color;
pub use options::{Command, Options};
pub use seed::Seed;
pub use state::{Apply, State};
pub use state_iter::StateIter;
pub use themes::{ThemeRef, Themes};

#[derive(clap::Args, Debug)]
pub struct CommonOpts {
    #[clap(long)]
    debug: bool,
}
