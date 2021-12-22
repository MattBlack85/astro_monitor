use minreq::{post, Error};
use std::{process, thread, time};
use structopt::StructOpt;
use sysinfo::{System, SystemExt};

static INTERVAL: time::Duration = time::Duration::from_secs(15);
static HOST: &'static str = "http://astromatto.com:11111/hook";

#[derive(StructOpt)]
struct CliArgs {
    api_token: String,
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

fn main() -> Result<(), Error> {
    let args = CliArgs::from_args();
    let api_token = args.api_token;
    let mut found: bool = false;

    println!("Looking for Kstars!");
    
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
