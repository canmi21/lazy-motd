/* src/env.rs */

use std::io::Write;
use sysinfo::{Disks, System};

pub fn print_environment(stdout: &mut dyn Write, mode: &str) -> std::io::Result<()> {
    let sys = System::new_all();

    // --- Mode ---
    if mode != "None" {
        let stage = if mode.is_empty() {
            std::env::var("STAGE").unwrap_or_else(|_| "Production".to_string())
        } else {
            mode.to_string()
        };
        writeln!(stdout, "    ✓ {}", stage)?;
    }

    // --- OS ---
    let mut os_info =
        System::long_os_version().unwrap_or_else(|| System::os_version().unwrap_or_default());
    if os_info.starts_with("MacOS") {
        os_info = os_info.replace("MacOS", "macOS");
    }
    let kernel_name = System::name().unwrap_or_default().to_lowercase();
    let kernel_version = System::kernel_version().unwrap_or_default();
    writeln!(
        stdout,
        "    ✓ {} {}",
        os_info,
        format!("{} {}", kernel_name, kernel_version)
    )?;

    // --- CPU ---
    let cpus = sys.cpus();
    let cpu_brand = cpus.first().map(|cpu| cpu.brand().trim()).unwrap_or("");
    let core_count = cpus.len();
    let arch = {
        let a = System::cpu_arch();
        if a.is_empty() {
            "Unknown Arch".to_string()
        } else {
            a
        }
    };

    // --- Disk ---
    let disks = Disks::new_with_refreshed_list();
    let fs_type = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .and_then(|d| d.file_system().to_str())
        .unwrap_or("Unknown FS");

    // --- Memory ---
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    let total_ram_gb = (sys.total_memory() as f64 / GIB).round() as u64;
    let used_ram_percent =
        (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0).round() as u64;

    writeln!(
        stdout,
        "    ✓ {}({}) {} {} {}GB {}%",
        cpu_brand, core_count, arch, fs_type, total_ram_gb, used_ram_percent
    )?;

    // --- Machine ID ---
    let fid = machine_uid::get().unwrap_or_else(|_| "Unavailable".to_string());
    writeln!(stdout, "    ✓ {}", fid)?;

    Ok(())
}
