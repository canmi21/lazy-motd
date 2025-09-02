/* examples/demo.rs */

fn main() {
    lazy_motd::lazy_motd!();
    lazy_motd::lazy_motd!(build = "(Nightly)");
    lazy_motd::lazy_motd!(build = "None");
    lazy_motd::lazy_motd!(bin = "my_custom_bin");
    lazy_motd::lazy_motd!(bin = "my_bin", build = "Nightly");
}
