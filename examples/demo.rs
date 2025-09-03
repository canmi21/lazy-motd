/* examples/demo.rs */

fn main() {
    lazy_motd::lazy_motd!();
    lazy_motd::lazy_motd!(build = "Nightly");
    lazy_motd::lazy_motd!(build = "None");
    lazy_motd::lazy_motd!(bin = "Custom");
    lazy_motd::lazy_motd!(bin = "None");
    lazy_motd::lazy_motd!(bin = "Example", build = "Nightly");
    lazy_motd::lazy_motd!(bin = "Example", build = "Nightly", timestamp = "None");
}
