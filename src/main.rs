use minreq::{post, Error};
use std::process::{Command, Stdio};
use std::{process, thread, time};
use structopt::StructOpt;
use sysinfo::{System, SystemExt};

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

fn check_lsof_on_system() -> bool {
    let result = Command::new("which").arg("lsof").output().expect("error");
    let output = String::from_utf8(result.stdout)
        .expect("fail to parse output")
        .to_string();
    match output.as_ref() {
        "" => return false,
        _ => return true,
    };
}

fn fd_report() {
    /// Runs lsof on the whole filesystem, sorting results, dropping duplicates and returns
    /// the top 20 offenders.
    ///
    /// This is equal to run `lsof -w / | awk '{print $1}' | sort | uniq -c | sort -r -n | head -n 20
    let lsof = Command::new("lsof")
        .arg("-w")
        .arg("/")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed echo command");

    let awk = Command::new("awk")
        .arg("{print $1}")
        .stdin(lsof.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("AWK error");

    let sort = Command::new("sort")
        .stdin(awk.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Sort failed");

    let uniq = Command::new("uniq")
        .arg("-c")
        .stdin(sort.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Uniq failed");

    let sort_again = Command::new("sort")
        .args(["-r", "-n"])
        .stdin(uniq.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Fail sort again");

    let head = Command::new("head")
        .args(["-n", "20"])
        .stdin(sort_again.stdout.unwrap())
        .output()
        .expect("Head failed");

    let report = String::from_utf8_lossy(&head.stdout);

    println!("{}", report);
}

fn main() -> Result<(), Error> {
    let args = CliArgs::from_args();
    let api_token = args.api_token;
    let fd_monitor = args.fd_monitor;
    let mut found: bool = false;

    if fd_monitor {
        match check_lsof_on_system() {
            false => {
                println!("`--fd-monitor` flag passed but `lsof` command not found or not available in PATH, aborting!");
                process::exit(0)
            }
            true => (),
        };
    }

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

        match fd_monitor {
            true => fd_report(),
            false => (),
        };

        thread::sleep(INTERVAL);
    }
}
