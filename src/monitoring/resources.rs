extern crate chrono;

use chrono::SecondsFormat;
use std::fs::OpenOptions;
use std::io::Write;
use sysinfo::{ProcessorExt, System, SystemExt};

pub fn cpu_and_memory(file_name: &String, log_path: &String) {
    let mut system = System::new();
    let mut options = OpenOptions::new();
    let log_file = format!("{}/system-{}.log", log_path, file_name);

    // Load CPU and memory usage
    system.refresh_system();

    // Store how much RAM is being used, divided by 1000 to give MB result
    let ram_usage: u64 = system.used_memory() / 1000;
    let processor_usage: f32 = system.global_processor_info().cpu_usage();
    let now = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);

    let mut file = match options.write(true).create(true).append(true).open(log_file) {
        Ok(f) => f,
        Err(e) => panic!("Error with file: {}", e),
    };

    let log = format!(
        "[{}] system -> CPU: {:.1}% | RAM: {}MB\n",
        now, processor_usage, ram_usage
    );

    match file.write_all(log.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("Error writing to file: {}", e),
    };
}
