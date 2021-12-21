use std::{process, thread, time};
use sysinfo::{System, SystemExt};
use structopt::StructOpt;
use minreq::{post, Error};

static INTERVAL: time::Duration = time::Duration::from_secs(15);
static HOST: &'static str = "http://astromatto.com:8000/hook";

#[derive(StructOpt)]
struct CliArgs {
    /// The pattern to look for
    api_token: String,
}

fn notify_via_telegram(token: &String) -> Result<(), Error> {
    let _response = post(format!("{}/{}", &HOST, &token)).send()?;
    process::exit(0);
}

fn main() -> Result<(), Error> {
    let args = CliArgs::from_args();
    let api_token = args.api_token;

    loop {
	let all_procs = System::new_all();
	let kstars_proc = all_procs.process_by_name("kstars");
	if kstars_proc.is_empty() {
	    let _resp = notify_via_telegram(&api_token)?;
	};
	thread::sleep(INTERVAL);
    }
}
