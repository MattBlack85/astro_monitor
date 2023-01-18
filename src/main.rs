#![crate_name = "astromonitor"]
extern crate chrono;
extern crate minreq;
extern crate structopt;
extern crate sysinfo;

use chrono::SecondsFormat;
use minreq::{post, Error};
use std::process::{Command, Stdio};
use std::{process, thread, time};
use structopt::StructOpt;
use sysinfo::{ProcessExt, System, SystemExt};
mod checks;
mod monitoring;

static INTERVAL: time::Duration = time::Duration::from_secs(15);
static HOST: &'static str = "http://astromatto.com:11111/hook";

#[derive(StructOpt)]
struct CliArgs {
    api_token: String,
    #[structopt(long)]
    fd_monitor: bool,
    #[structopt(long)]
    system_monitor: bool,
}

struct Paths {
    folder_path: String,
    logs_path: String,
    home_path: String,
}

impl Paths {
    fn logs_full_path(&self) -> String {
        let logs_full_path = format!("{}/{}", self.root_path(), self.logs_path);
        return logs_full_path;
    }

    fn root_path(&self) -> String {
        let root_path = format!("{}/{}", self.home_path, self.folder_path);
        return root_path;
    }

    fn init() -> Self {
        Self {
            folder_path: String::from(".local/share/astromonitor"),
            logs_path: String::from("logs"),
            home_path: dirs::home_dir().unwrap().as_path().display().to_string(),
        }
    }
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

/// Runs lsof on the whole filesystem, sorting results, dropping duplicates and returns
/// the top 20 offenders.
///
/// This is equal to run `lsof -w / | awk '{print $1}' | sort | uniq -c | sort -r -n | head -n 20
fn fd_report() {
    let lsof = Command::new("lsof")
        .arg("-w")
        .arg("/")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed echo command");

    let awk = Command::new("awk")
        .arg("{print $1 \" \" $2}")
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
        .output()
        .expect("Fail sort again");

    let report = String::from_utf8_lossy(&sort_again.stdout);

    println!("{}", report);
}

fn main() -> Result<(), Error> {
    let args = CliArgs::from_args();
    let api_token = args.api_token;
    let fd_monitor = args.fd_monitor;
    let kstars = args.kstars;
    let paths = Paths::init();

    // Boostrap the main folder where logs and our things will be stored
    match checks::vault::build_astromonitor_folder_tree() {
        Ok(()) => (),
        Err(e) => panic!(
            "The folder to store logs cannot be created, reason => {}",
            e
        ),
    };

    if fd_monitor {
        match checks::system::lsof_on_system() {
            false => {
                println!("`--fd-monitor` flag passed but `lsof` command not found or not available in PATH, aborting!");
                process::exit(0)
            }
            true => (),
        };
    }

    if kstars {
        println!("Looking for Kstars!");

        let mut system = System::new();
        let start_time: String = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);

        // Check if Kstars is already running when astromonitor starts and store the pid
        // if that's the case, otherwise exit gracefully.
        system.refresh_processes();
        system.process_by_exact_name("kstars");
        let kstars = system.process_by_exact_name("kstars");
        if kstars.is_empty() {
            println!("It seems you haven't started Kstars yet, please do it first.");
            process::exit(0);
        } else {
            let _pid: i32 = kstars[0].pid();
        };

        loop {
            system.refresh_processes();
            let kstars_proc = system.process_by_exact_name("kstars");
            if kstars_proc.is_empty() {
                println!("Kstars stopped running, sending a notification");
                notify_via_telegram(&api_token);
            };

            match fd_monitor {
                true => fd_report(),
                false => (),
            };

            match args.system_monitor {
                true => monitoring::resources::cpu_and_memory(&start_time, &paths.logs_full_path()),
                false => (),
            }

            thread::sleep(INTERVAL);
        }
    };
}
