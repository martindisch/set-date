use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The directory to work on.
    directory: PathBuf,

    /// Doesn't write any changes to files.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}")
}
