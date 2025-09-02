/* src/lib.rs */

#[macro_export]
macro_rules! lazy_motd {
    (bin=$bin_name:literal, build=$build_info:literal) => {{
        use ::std::io::Write;
        use ::termcolor::WriteColor;
        let print_motd = || -> ::std::io::Result<()> {
            let mut stdout = ::termcolor::StandardStream::stdout(::termcolor::ColorChoice::Auto);
            let pkg_name = if $bin_name == "None" {
                env!("CARGO_BIN_NAME")
            } else if $bin_name != "" {
                $bin_name
            } else {
                env!("CARGO_PKG_NAME")
            };
            let pkg_version = env!("CARGO_PKG_VERSION");
            let timestamp = ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(&mut stdout)?;
            stdout.set_color(
                ::termcolor::ColorSpec::new().set_fg(Some(::termcolor::Color::Magenta)),
            )?;
            write!(&mut stdout, "  ▲ {} {}", pkg_name, pkg_version)?;
            stdout.reset()?;
            if $build_info != "" && $build_info != "None" {
                if $build_info.starts_with("(") {
                    write!(&mut stdout, " {}", $build_info)?;
                } else {
                    write!(&mut stdout, " ({})", $build_info)?;
                }
            }
            writeln!(&mut stdout)?;
            writeln!(&mut stdout, "  - Timestamp: {}", timestamp)?;
            writeln!(&mut stdout)?;
            Ok(())
        };
        if let Err(e) = print_motd() {
            eprintln!("Failed to print motd: {}", e);
        }
    }};
    () => {
        $crate::lazy_motd!(bin = "", build = "Preview");
    };
    (build=$build_info:literal) => {
        $crate::lazy_motd!(bin = "", build = $build_info);
    };
    (bin=$bin_name:literal) => {
        $crate::lazy_motd!(bin = $bin_name, build = "Preview");
    };
}
