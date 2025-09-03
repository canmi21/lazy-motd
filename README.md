# lazy-motd

The perfect MOTD for the elegantly lazy developer. Maximum style, minimal effort.

## Overview

`lazy-motd` is a Rust crate that provides a simple, customizable Message of the Day (MOTD) for your command-line applications. With minimal configuration, it displays stylish system information, including package details, build info, environment stats, and copyright notices, all formatted with colorful terminal output.

## Features

- **Easy to Use**: A single macro `lazy_motd!()` to generate a professional MOTD.
- **Customizable**: Configure package name, build info, mode, timestamp, environment, and copyright details.
- **System Info**: Displays OS, CPU, disk, memory, and machine ID with minimal effort.
- **Colorful Output**: Uses `termcolor` for vibrant, terminal-friendly formatting.
- **Lightweight**: Minimal dependencies for a lean crate.

## Installation

Add `lazy-motd` to your `Cargo.toml`:

```toml
[dependencies]
lazy-motd = "1"
```

## Usage

Basic usage with default settings:

```rust
use lazy_motd::lazy_motd;

fn main() {
    lazy_motd!();
}
```

Customized MOTD with additional options:

```rust
use lazy_motd::lazy_motd;

fn main() {
    lazy_motd!(
        bin = "MyApp",
        build = "Release",
        mode = "Production",
        timestamp = "",
        environment = "",
        copyright = &["Copyright (c) 2025 My Company", "All rights reserved."]
    );
}
```

## Example Output

Running the above code might produce:

```
  ▲ MyApp 1.0.7 (Release)
  - Timestamp: 2025-09-03 09:49:23
  - Copyright:
    ✓ Copyright (c) 2025 My Company
    ✓ All rights reserved.
  - Environment:
    ✓ Production
    ✓ macOS 14.6.1 darwin 23.5.0
    ✓ Apple M1(8) arm64 apfs 16GB 45%
    ✓ 123e4567-e89b-12d3-a456-426614174000
```

## Building and Running Examples

To try the included demo:

```bash
cargo run --example demo
```

## Dependencies

- `chrono`: For timestamp formatting.
- `machine-uid`: For unique machine ID retrieval.
- `sysinfo`: For system information (OS, CPU, memory, disk).
- `termcolor`: For colored terminal output.

## License

Licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on the [GitHub repository](https://github.com/yourusername/lazy-motd).
