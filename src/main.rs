use std::{fmt::Display, path::PathBuf};

use clap::Parser;
use eyre::{eyre, OptionExt, Result};
use once_cell::sync::Lazy;
use regex::Regex;
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
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let filename = entry.file_name().to_str().ok_or_eyre("Invalid filename")?;
        let date = infer_date(filename)?;
        println!("{filename}: {date}")
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

/// Infers the date from a pattern such as
/// - 1996-05 Martin
/// - 1999-08-24 Caschurn
/// - 2003-07-12..13 Malbun Pfläzerhütte
/// - 2002-08-16Maighelshütte Tomasee
/// and returns it in EXIF format `YYYY:MM:DD HH:MM:SS`.
fn infer_date(filename: &str) -> Result<ExifDateTime> {
    static REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"([0-9]{4})-([0-9]{2})-?([0-9]{2})?").unwrap());

    let captures = REGEX
        .captures(filename)
        .ok_or_else(|| eyre!("No matches found for {filename}"))?;
    let year = captures.get(1);
    let month = captures.get(2);
    let day = captures.get(3);

    let date = match (year, month, day) {
        (Some(year), Some(month), Some(day)) => Ok(ExifDateTime {
            year: year.as_str(),
            month: month.as_str(),
            day: Some(day.as_str()),
        }),
        (Some(year), Some(month), None) => Ok(ExifDateTime {
            year: year.as_str(),
            month: month.as_str(),
            day: None,
        }),
        _ => Err(eyre!("Invalid date format: {}", filename)),
    };

    date
}

#[derive(Debug)]
struct ExifDateTime<'a> {
    year: &'a str,
    month: &'a str,
    day: Option<&'a str>,
}

impl Display for ExifDateTime<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{} 00:00:00",
            self.year,
            self.month,
            self.day.unwrap_or("01")
        )
    }
}
