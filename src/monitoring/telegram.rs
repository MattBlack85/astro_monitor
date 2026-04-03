use astromonitor::HOST;
use log::{error, info, warn};
use std::process;

pub fn notify_via_telegram(token: &String) {
    match minreq::post(format!("{}/hook/{}", HOST, token)).send() {
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
    }
}
