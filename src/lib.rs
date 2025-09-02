/* src/lib.rs */

#[macro_export]
macro_rules! lazy_motd {
    ($build_info:literal) => {{
        use ::std::io::Write;
        use ::termcolor::WriteColor;
        let print_motd = || -> ::std::io::Result<()> {
            let mut stdout = ::termcolor::StandardStream::stdout(::termcolor::ColorChoice::Auto);
            let pkg_name = env!("CARGO_PKG_NAME");
            let pkg_version = env!("CARGO_PKG_VERSION");
            let timestamp = ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(&mut stdout)?;
            stdout.set_color(
                ::termcolor::ColorSpec::new().set_fg(Some(::termcolor::Color::Magenta)),
            )?;
            write!(&mut stdout, "  ▲ {} {}", pkg_name, pkg_version)?;
            stdout.reset()?;
            if $build_info != "None" {
                write!(&mut stdout, " {}", $build_info)?;
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
        $crate::lazy_motd!("(Preview)");
    };
}
