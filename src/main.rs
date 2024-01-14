use std::path::PathBuf;

use clap::Parser;
use eyre::Result;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The directory to work on.
    directory: PathBuf,

    /// Doesn't write any changes to files.
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    for entry in WalkDir::new(args.directory)
        .into_iter()
        .filter_entry(|e| !should_skip(e))
    {
        println!("{}", entry?.path().display());
    }

    Ok(())
}

fn should_skip(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") || s == "Thumbs.db")
        .unwrap_or(true)
}
