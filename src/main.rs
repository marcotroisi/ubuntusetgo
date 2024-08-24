use anyhow::{Context, Result};
use clap::Parser;
use console::{style, Emoji};
use indicatif::ProgressBar;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,
    /// The pattern to look for
    pattern: Option<String>,
}

static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static FOUND: Emoji<'_, '_> = Emoji("✅  ", "");

fn main() -> Result<()> {
    let args = Cli::parse();

    println!(
        "{} {} Using file {}",
        PAPER,
        style("[Config]").bold().dim(),
        args.path.display()
    );

    if let Some(pattern) = &args.pattern {
        println!(
            "{} {} Searching for '{}'",
            LOOKING_GLASS,
            style("[Section]").bold().dim(),
            pattern
        );
    }

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    println!("\n");
    let pb = ProgressBar::new(content.lines().count() as u64);
    for line in content.lines() {
        if let Some(pattern) = &args.pattern {
            let section = format!("[{}]", pattern);
            if line.contains(&section) {
                println!("{} {}", line, FOUND);
            }
        }
        pb.inc(1);
    }
    pb.finish();
    Ok(())
}
