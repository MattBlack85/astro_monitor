use minreq::{post, Error};
use std::{process, thread, time};
use structopt::StructOpt;
use sysinfo::{System, SystemExt};
use std::process::Command;

static INTERVAL: time::Duration = time::Duration::from_secs(15);
static HOST: &'static str = "http://astromatto.com:11111/hook";

#[derive(StructOpt)]
struct CliArgs {
    api_token: String,
    #[structopt(long)]
    fd_monitor: bool,
}

fn notify_via_telegram(token: &String) -> Result<(), Error> {
    let response = post(format!("{}/{}", &HOST, &token)).send()?;
    
    println!("Kstars stopped running, sending a notification");
    
    process::exit(match response.status_code {
        200 => {
            println!("Notification sent! Bye!");
            0
        }
        _ => {
            println!("Notification failed with status: {}", response.status_code);
            1
        }
    });
}

fn check_lsof_on_system() -> u8 {
    let result = Command::new("which")
        .arg("lsof")
        .output()
	.expect("error");
    let output = String::from_utf8(result.stdout).expect("fail to parse output").to_string();
    match output.as_ref() {
	"" => return 0,
	_ => return 1,
    };
	
}

fn main() -> Result<(), Error> {
    let args = CliArgs::from_args();
    let api_token = args.api_token;
    let fd_monitor = args.fd_monitor;
    let mut found: bool = false;

    if fd_monitor {
	let res = check_lsof_on_system();
	if res == 0 {
	    println!("`--fd-monitor flag` passed but `lsof` command not found or not available in PATH, aborting!");
	    process::exit(0);
	}
    }

    println!("Looking for Kstars!");
    
    // Find a way to run lsof / | awk '{print $1}' | sort | uniq -c | sort -r -n | head -n 20 periodically
    loop {
        let all_procs = System::new_all();
        let kstars_proc = all_procs.process_by_name("kstars");
        if kstars_proc.is_empty() {
            let _resp = notify_via_telegram(&api_token)?;
        };
	match found {
	    true => (),
	    false => println!("Kstars found! Started monitoring"),
	};
	found = true;
        thread::sleep(INTERVAL);
    }
}
