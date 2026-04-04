#![crate_name = "astromonitor"]

mod backup;
mod checks;
mod monitoring;
mod notifications;
mod tui;

fn main() {
    if let Err(e) = tui::runner::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
