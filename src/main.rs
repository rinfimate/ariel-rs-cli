use ariel_rs::{render, theme::Theme};
use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process;

/// arielc — fast Mermaid diagram renderer.
///
/// Drop-in replacement for mmdc (mermaid-cli). Renders .mmd files to SVG
/// without Node.js, Puppeteer, or a browser.
#[derive(Parser)]
#[command(
    name = "arielc",
    version,
    about = "Fast Mermaid diagram renderer — drop-in replacement for mmdc",
    long_about = None
)]
struct Args {
    /// Input .mmd file (use - for stdin)
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Output file (.svg). Use - for stdout
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    /// Theme: default | dark | forest | neutral
    #[arg(short, long, default_value = "default")]
    theme: String,

    /// Background colour (e.g. transparent, #ffffff)
    #[arg(short, long, value_name = "COLOR")]
    background_color: Option<String>,

    /// Suppress all output
    #[arg(long)]
    quiet: bool,
}

fn parse_theme(s: &str) -> Theme {
    match s.to_lowercase().as_str() {
        "dark" => Theme::Dark,
        "forest" => Theme::Forest,
        "neutral" => Theme::Neutral,
        _ => Theme::Default,
    }
}

fn main() {
    let args = Args::parse();

    // Read input
    let input = if args.input.to_str() == Some("-") {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .unwrap_or_else(|e| { eprintln!("arielc: failed to read stdin: {e}"); process::exit(1); });
        buf
    } else {
        fs::read_to_string(&args.input).unwrap_or_else(|e| {
            eprintln!("arielc: cannot read '{}': {e}", args.input.display());
            process::exit(1);
        })
    };

    // Render
    let theme = parse_theme(&args.theme);
    let svg = render(&input, theme);

    // Write output
    if args.output.to_str() == Some("-") {
        io::stdout()
            .write_all(svg.as_bytes())
            .unwrap_or_else(|e| { eprintln!("arielc: write error: {e}"); process::exit(1); });
    } else {
        fs::write(&args.output, &svg).unwrap_or_else(|e| {
            eprintln!("arielc: cannot write '{}': {e}", args.output.display());
            process::exit(1);
        });

        if !args.quiet {
            eprintln!("arielc: wrote {}", args.output.display());
        }
    }
}
