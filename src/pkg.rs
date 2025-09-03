/* src/pkg.rs */

use chrono::Local;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[macro_export]
macro_rules! resolve_pkg_name {
    ($bin_name:expr) => {
        if $bin_name == "None" {
            env!("CARGO_BIN_NAME")
        } else if $bin_name != "" {
            $bin_name
        } else {
            env!("CARGO_PKG_NAME")
        }
    };
}

pub fn print_motd(pkg_name: &str, build_info: &str, timestamp: &str) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    let pkg_version = env!("CARGO_PKG_VERSION");
    let ts_string = if timestamp == "None" {
        String::new()
    } else {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    };

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
    if !ts_string.is_empty() {
        writeln!(&mut stdout, "  - Timestamp: {}", ts_string)?;
    }
    writeln!(&mut stdout)?;
    Ok(())
}
