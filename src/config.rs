use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Where alerts should be delivered.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum NotificationMode {
    /// Through the Telegram relay. Requires internet.
    #[default]
    Telegram,
    /// As a UDP datagram on the local network. Works with no internet.
    Lan,
    /// Both; the alert counts as delivered if either one gets through.
    Both,
}

impl NotificationMode {
    pub fn label(&self) -> &'static str {
        match self {
            NotificationMode::Telegram => "Telegram",
            NotificationMode::Lan => "LAN",
            NotificationMode::Both => "Both",
        }
    }

    pub fn uses_telegram(&self) -> bool {
        matches!(self, NotificationMode::Telegram | NotificationMode::Both)
    }

    pub fn uses_lan(&self) -> bool {
        matches!(self, NotificationMode::Lan | NotificationMode::Both)
    }
}

/// Default target for LAN delivery: a broadcast on the observing network.
pub const DEFAULT_LAN_ADDR: &str = "255.255.255.255:5005";

fn default_lan_addr() -> String {
    DEFAULT_LAN_ADDR.to_string()
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub token: String,
    /// Both fields carry `serde(default)` so that a config file written by an
    /// earlier version — which only had `token` — still deserialises.
    #[serde(default)]
    pub notification_mode: NotificationMode,
    #[serde(default = "default_lan_addr")]
    pub lan_broadcast_addr: String,
}

impl AppConfig {
    /// A config with only a Telegram token, as produced before LAN delivery
    /// existed.
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            notification_mode: NotificationMode::default(),
            lan_broadcast_addr: default_lan_addr(),
        }
    }
}

pub fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("could not determine config directory");
    path.push("astromonitor");
    path.push("astro.json");
    path
}

pub fn load_config() -> Option<AppConfig> {
    let path = config_path();
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_config_still_deserialises() {
        // Exactly what earlier versions wrote to astro.json.
        let legacy = r#"{"token": "abc123"}"#;
        let cfg: AppConfig = serde_json::from_str(legacy).expect("legacy config must load");
        assert_eq!(cfg.token, "abc123");
        assert_eq!(cfg.notification_mode, NotificationMode::Telegram);
        assert_eq!(cfg.lan_broadcast_addr, "255.255.255.255:5005");
    }

    #[test]
    fn mode_round_trips_through_json() {
        let cfg = AppConfig {
            token: "t".to_string(),
            notification_mode: NotificationMode::Both,
            lan_broadcast_addr: "127.0.0.1:5005".to_string(),
        };
        let json = serde_json::to_string(&cfg).unwrap();
        assert!(json.contains(r#""notification_mode":"both""#));
        let back: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.notification_mode, NotificationMode::Both);
        assert_eq!(back.lan_broadcast_addr, "127.0.0.1:5005");
    }

    #[test]
    fn mode_predicates() {
        assert!(NotificationMode::Telegram.uses_telegram());
        assert!(!NotificationMode::Telegram.uses_lan());
        assert!(NotificationMode::Lan.uses_lan());
        assert!(!NotificationMode::Lan.uses_telegram());
        assert!(NotificationMode::Both.uses_telegram());
        assert!(NotificationMode::Both.uses_lan());
    }
}
