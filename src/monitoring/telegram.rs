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

/// Send a KStars-stopped alert via Telegram without exiting the process.
/// Returns Ok(()) on HTTP 200, Err with a description otherwise.
pub fn send_kstars_alert(token: &str) -> Result<(), String> {
    match minreq::post(format!("{}/hook/{}", HOST, token)).send() {
        Ok(r) if r.status_code == 200 => {
            info!("KStars alert sent successfully.");
            Ok(())
        }
        Ok(r) => {
            warn!("KStars alert: unexpected status {}", r.status_code);
            Err(format!("HTTP status {}", r.status_code))
        }
        Err(e) => {
            error!("KStars alert: request failed: {}", e);
            Err(e.to_string())
        }
    }
}
