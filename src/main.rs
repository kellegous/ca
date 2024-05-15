use ca::Command;

use clap::Parser;

use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    args.command.run()?;
    Ok(())
}
