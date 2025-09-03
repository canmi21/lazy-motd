/* src/lib.rs */

pub mod env;
pub mod pkg;

#[macro_export]
macro_rules! lazy_motd {
    (copyright = [$($copyright_line:expr),* $(,)?] $(, $key:ident = $val:expr)* $(,)?) => {{
        let copyright_lines: &[&str] = &[$($copyright_line),*];
        $crate::lazy_motd!(copyright = copyright_lines $(, $key = $val)*);
    }};

    ($( $key:ident = $val:expr ),* $(,)?) => {{
        let mut bin: Option<&str> = None;
        let mut build: Option<&str> = None;
        let mut mode: Option<&str> = None;
        let mut timestamp: Option<&str> = None;
        let mut environment: Option<&str> = None;
        let mut copyright: Option<&[&str]> = None;

        $(
            lazy_motd!(@assign $key = $val, bin, build, mode, timestamp, environment, copyright);
        )*
        let final_pkg_name = if let Some(b) = bin {
            if b == "None" {
                // If bin is "None", use the raw binary name without formatting.
                env!("CARGO_BIN_NAME").to_string()
            } else if !b.is_empty() {
                // If a custom bin name is provided, format it.
                $crate::pkg::capitalize_first(b)
            } else {
                // If bin is empty, use the default package name and format it.
                $crate::pkg::capitalize_first(env!("CARGO_PKG_NAME"))
            }
        } else {
            // If bin is not specified, use the default package name and format it.
            $crate::pkg::capitalize_first(env!("CARGO_PKG_NAME"))
        };

        if let Err(e) = $crate::pkg::print_motd(
            &final_pkg_name, // Pass the processed name
            build.unwrap_or("Preview"),
            $crate::pkg::MotdOptions {
                mode,
                timestamp,
                environment,
                copyright,
            }
        ) {
            eprintln!("Failed to print motd: {}", e);
        }
    }};

    (@assign bin = $val:expr, $bin:ident, $build:ident, $mode:ident, $timestamp:ident, $environment:ident, $copyright:ident) => {
        $bin = Some($val);
    };
    (@assign build = $val:expr, $bin:ident, $build:ident, $mode:ident, $timestamp:ident, $environment:ident, $copyright:ident) => {
        $build = Some($val);
    };
    (@assign mode = $val:expr, $bin:ident, $build:ident, $mode:ident, $timestamp:ident, $environment:ident, $copyright:ident) => {
        $mode = Some($val);
    };
    (@assign timestamp = $val:expr, $bin:ident, $build:ident, $mode:ident, $timestamp:ident, $environment:ident, $copyright:ident) => {
        $timestamp = Some($val);
    };
    (@assign environment = $val:expr, $bin:ident, $build:ident, $mode:ident, $timestamp:ident, $environment:ident, $copyright:ident) => {
        $environment = Some($val);
    };
    (@assign copyright = $val:expr, $bin:ident, $build:ident, $mode:ident, $timestamp:ident, $environment:ident, $copyright:ident) => {
        $copyright = Some($val);
    };

    () => {
        $crate::lazy_motd!(bin = "", build = "Preview");
    };
}
