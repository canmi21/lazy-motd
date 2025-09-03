/* examples/demo.rs */

use lazy_motd::lazy_motd;

fn main() {
    lazy_motd!();
    lazy_motd!(build = "Nightly");
    lazy_motd!(build = "None");
    lazy_motd!(bin = "Custom");
    lazy_motd!(bin = "None");
    lazy_motd!(bin = "Example", build = "Nightly");
    lazy_motd!(bin = "Example", build = "Nightly", timestamp = "None");

    lazy_motd!(
        bin = "Example",
        build = "Production",
        environment = "",
        mode = "Development"
    );

    lazy_motd!(
        bin = "MyApp",
        build = "Release",
        copyright = &["Copyright (c) 2025 My Company", "All rights reserved."]
    );

    lazy_motd!(
        bin = "FullExample",
        build = "Stable",
        mode = "Production",
        timestamp = "",
        environment = "",
        copyright = &[
            "Copyright (c) 2025 Example Corp",
            "Licensed under MIT License"
        ]
    );
}
