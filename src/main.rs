#![crate_name = "astromonitor"]
extern crate chrono;
extern crate minreq;
extern crate structopt;
extern crate sysinfo;

use astromonitor::{CliArgs, Paths, HOST, INTERVAL};
use chrono::SecondsFormat;
use std::process::{Command, Stdio};
use std::{process, thread};
use structopt::StructOpt;
use sysinfo::{ProcessExt, System, SystemExt};

mod backup;
mod checks;
mod monitoring;

fn notify_via_telegram(token: &String) {
    match minreq::post(format!("/hook/{}/{}", &HOST, &token)).send() {
        Ok(r) => {
            process::exit(match r.status_code {
                200 => {
                    println!("Notification sent! Bye!");
                    0
                }
                _ => {
                    println!("Notification failed with status: {}", r.status_code);
                    1
                }
            });
        }
        Err(e) => {
            println!("The request couldn't be sent to the bot: {}", e);
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
    let args = CliArgs::from_args();
    let api_token = args.api_token;
    let fd_monitor = args.fd_monitor;
    let do_backup = args.do_backup;
    let kstars = args.kstars;
    let paths = Paths::init();

    // Boostrap the main folder where logs and our things will be stored
    match checks::vault::build_astromonitor_folder_tree() {
        Ok(()) => (),
        Err(e) => println!(
            "The folder to store logs cannot be created, reason => {}",
            e
        ),
    };

    if do_backup {
        match backup::database::send_db(&paths, &api_token) {
            Ok(_) => (),
            Err(s) => println!("{}", format!("Error while trying to make a backup: {}", s)),
        }
    }

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
