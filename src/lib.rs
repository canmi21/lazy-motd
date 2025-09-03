/* src/lib.rs */

pub mod pkg;

#[macro_export]
macro_rules! lazy_motd {
    (bin=$bin_name:literal, build=$build_info:literal, timestamp=$ts:literal) => {{
        let pkg_name = $crate::resolve_pkg_name!($bin_name);
        if let Err(e) = $crate::pkg::print_motd(pkg_name, $build_info, $ts) {
            eprintln!("Failed to print motd: {}", e);
        }
    }};
    (bin=$bin_name:literal, build=$build_info:literal) => {
        $crate::lazy_motd!(bin = $bin_name, build = $build_info, timestamp = "");
    };
    () => {
        $crate::lazy_motd!(bin = "", build = "Preview", timestamp = "");
    };
    (build=$build_info:literal) => {
        $crate::lazy_motd!(bin = "", build = $build_info, timestamp = "");
    };
    (bin=$bin_name:literal) => {
        $crate::lazy_motd!(bin = $bin_name, build = "Preview", timestamp = "");
    };
}
