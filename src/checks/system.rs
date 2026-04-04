use std::process::Command;
use sysinfo::{System, SystemExt};

pub fn lsof_on_system() -> bool {
    let result = Command::new("which").arg("lsof").output().expect("error");
    let output = String::from_utf8(result.stdout)
        .expect("fail to parse output")
        .to_string();
    match output.as_ref() {
        "" => return false,
        _ => return true,
    };
}

/// Returns true if a process named "kstars" is currently running.
pub fn kstars_is_running() -> bool {
    let mut system = System::new();
    system.refresh_processes();
    let found = system.processes_by_exact_name("kstars").next().is_some();
    found
}
