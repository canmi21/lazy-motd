/* src/lib.rs */

pub mod pkg;

#[macro_export]
macro_rules! lazy_motd {
    (bin=$bin_name:literal, build=$build_info:literal) => {{
        let pkg_name = $crate::resolve_pkg_name!($bin_name);
        if let Err(e) = $crate::pkg::print_motd(pkg_name, $build_info) {
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
