mod toml_parser;

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
    if let Some(pattern) = &args.pattern {
        find_matches(&content, pattern, &mut std::io::stdout());
        pb.inc(1);
    }
    pb.finish();
    Ok(())
}

fn find_matches(content: &str, section: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        let section = format!("[{}]", section);
        if line.contains(&section) {
            writeln!(writer, "{} {}", line, FOUND).expect("failed to write line");
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("[os]\nsomething here", "os", &mut result);
    let expected = format!("[os] {}\n", FOUND).into_bytes();

    assert_eq!(
        result,
        expected,
        "find a match: expected [{:?}], got [{:?}]",
        String::from_utf8_lossy(&expected),
        String::from_utf8_lossy(&result)
    );
}
