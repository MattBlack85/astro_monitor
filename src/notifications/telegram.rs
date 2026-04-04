use super::Notification;
use astromonitor::HOST;
use log::{error, info, warn};

pub struct TelegramNotifier {
    pub token: String,
}

impl TelegramNotifier {
    pub fn new(token: impl Into<String>) -> Self {
        Self { token: token.into() }
    }
}

impl Notification for TelegramNotifier {
    fn send(&self, message: &str) -> Result<(), String> {
        match minreq::post(format!("{}/hook/{}", HOST, self.token))
            .with_body(message)
            .send()
        {
            Ok(r) if r.status_code == 200 => {
                info!("Telegram notification sent successfully.");
                Ok(())
            }
            Ok(r) => {
                warn!("Telegram notification: unexpected status {}", r.status_code);
                Err(format!("HTTP status {}", r.status_code))
            }
            Err(e) => {
                error!("Telegram notification: request failed: {}", e);
                Err(e.to_string())
            }
        }
    }
}
