/* src/pkg.rs */

use crate::env;
use chrono::Local;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Default)]
pub struct MotdOptions<'a> {
    pub mode: Option<&'a str>,        // "Development" | "Production" | "None"
    pub timestamp: Option<&'a str>,   // "None" | ""
    pub environment: Option<&'a str>, // "None" | ""
    pub copyright: Option<&'a [&'a str]>,
}

/// Formats a string to be all lowercase with the first letter capitalized.
pub fn capitalize_first(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut c = lower.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn print_motd(pkg_name: &str, build_info: &str, opts: MotdOptions) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let pkg_version = env!("CARGO_PKG_VERSION");

    writeln!(&mut stdout)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
    write!(&mut stdout, "  ▲ {} {}", pkg_name, pkg_version)?;
    stdout.reset()?;

    if !build_info.is_empty() && build_info != "None" {
        if build_info.starts_with("(") {
            write!(&mut stdout, " {}", build_info)?;
        } else {
            write!(&mut stdout, " ({})", build_info)?;
        }
    }
    writeln!(&mut stdout)?;

    // --- Timestamp ---
    if opts.timestamp.unwrap_or("") != "None" {
        let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
        writeln!(&mut stdout, "  - Timestamp: {}", ts)?;
    }

    // --- Copyright ---
    if let Some(lines) = opts.copyright {
        if !lines.is_empty() {
            writeln!(&mut stdout, "  - Copyright:")?;
            for l in lines {
                writeln!(&mut stdout, "    ✓ {}", l)?;
            }
        }
    }

    // --- Environment ---
    if opts.environment.unwrap_or("") != "None" {
        writeln!(&mut stdout, "  - Environment:")?;
        env::print_environment(&mut stdout, opts.mode.unwrap_or(""))?;
    }

    writeln!(&mut stdout)?;
    Ok(())
}
