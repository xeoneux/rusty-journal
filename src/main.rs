mod cli;
mod tasks;

use structopt::StructOpt;

fn main() {
    cli::CommandLineArgs::from_args();
}
