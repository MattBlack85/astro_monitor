use std::{thread, time};
use sysinfo::{System, SystemExt};

static INTERVAL: time::Duration = time::Duration::from_secs(15);

fn notify_via_telegram() {
    println!("notifying");
}

fn main() {
    loop {
	let all_procs = System::new_all();
	match all_procs.process_by_name("kstars")[..] {
	    [] => notify_via_telegram(),
	    [..] => (),
	};
	thread::sleep(INTERVAL);
    }
}
