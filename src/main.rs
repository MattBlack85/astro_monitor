#![crate_name = "astromonitor"]
extern crate chrono;
extern crate minreq;
extern crate structopt;
extern crate sysinfo;

use astromonitor::{CliArgs, Paths, HOST, INTERVAL};
use chrono::SecondsFormat;
use env_logger::Env;
use log::{error, info, warn};
use std::process::{Command, Stdio};
use std::{process, thread};
use structopt::StructOpt;
use sysinfo::{System, SystemExt};

mod backup;
mod checks;
mod monitoring;

fn notify_via_telegram(token: &String) {
    match minreq::post(format!("/hook/{}/{}", &HOST, &token)).send() {
        Ok(r) => {
            process::exit(match r.status_code {
                200 => {
                    info!("Notification sent! Bye!");
                    0
                }
                _ => {
                    warn!("Notification failed with status: {}", r.status_code);
                    1
                }
            });
        }
        Err(e) => {
            error!("The request couldn't be sent to the bot: {}", e);
        }
    };
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

fn main() {
    let env = Env::default().filter_or("ASTROMONITOR_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let args = CliArgs::from_args();
    let api_token = args.api_token;
    let fd_monitor = args.fd_monitor;
    let do_backup = args.do_backup;
    let retrieve_backup = args.retrieve_backup;
    let kstars = args.kstars;
    let paths = Paths::init();

    // Boostrap the main folder where logs and our things will be stored
    match checks::vault::build_astromonitor_folder_tree() {
        Ok(()) => (),
        Err(e) => {
            error!(
                "The folder to store logs cannot be created, reason => {}",
                e
            );
            panic!("Folder to store logs cannot be created, check the error and retry, bye!");
        }
    };

    if do_backup {
        match backup::database::send_db(&paths, &api_token) {
            Ok(_) => (),
            Err(s) => warn!("Error while trying to make a backup: {}", s),
        }
    }

    if retrieve_backup {
        match backup::database::retrieve_db(&paths, &api_token) {
            Ok(_) => (),
            Err(s) => warn!("Error while trying to make a backup: {}", s),
        }
    }

    if fd_monitor {
        match checks::system::lsof_on_system() {
            false => {
                error!("`--fd-monitor` flag passed but `lsof` command not found or not available in PATH, aborting!");
                process::exit(0)
            }
            true => (),
        };
    }

    if kstars {
        info!("Looking for Kstars!");

        let mut system = System::new();
        let start_time: String = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);

        // Check if Kstars is already running when astromonitor starts and store the pid
        // if that's the case, otherwise exit gracefully.
        let mut start_system = System::new();
        start_system.refresh_all();
        let mut start_procs = start_system.processes_by_exact_name("kstars");
        let check = start_procs.next();

        match check {
            Some(_) => {
                info!("Kstars found -- I am starting monitoring");
            }
            None => {
                warn!("It seems you haven't started Kstars yet, please do it first.");
                process::exit(0);
            }
        }

        loop {
            system.refresh_processes();
            let mut procs = system.processes_by_exact_name("kstars");
            let kstars = procs.next();

            match kstars {
                Some(_) => (),
                None => {
                    warn!("Kstars stopped running, sending a notification");
                    notify_via_telegram(&api_token);
                }
            }

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
