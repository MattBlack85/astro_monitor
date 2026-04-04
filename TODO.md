# TODO

## Issue #17 — LAN Push Notifications (no internet required)

**Feature request:** Support push notifications delivered over the local area network for users
observing at remote sites where internet access is weak or unavailable.

---

### Background

The current `TelegramNotifier` sends alerts through the external server at `astromatto.com`,
which requires internet. At a dark-sky site with no connectivity, KStars-stopped alerts are
silently lost.

Issue reference: https://github.com/MattBlack85/astro_monitor/issues/17

---

### Plan

#### 1. Add a `LanNotifier` backend (`src/notifications/lan.rs`)

- Implement the `Notification` trait on a `LanNotifier` struct.
- On `send()`, broadcast or unicast a UDP datagram on the LAN (e.g. port `5005`, configurable).
- A companion mobile/desktop listener app (or simple script) receives the datagram and surfaces
  an OS notification — no relay server needed.
- Use only `std::net::UdpSocket` (already in `std`); no new dependencies required.

```rust
pub struct LanNotifier {
    pub broadcast_addr: String,  // e.g. "255.255.255.255:5005"
}

impl Notification for LanNotifier {
    fn send(&self, message: &str) -> Result<(), String> { ... }
}
```

#### 2. Extend `AppConfig` (`src/config.rs`)

Add an optional `notification_mode` field so the user can choose their backend:

```json
{
  "token": "<telegram token>",
  "notification_mode": "telegram" | "lan" | "both",
  "lan_broadcast_addr": "255.255.255.255:5005"
}
```

Deserialize with `#[serde(default)]` so existing config files remain valid.

#### 3. Update the Setup wizard (`src/tui/screens/setup.rs` + `src/tui/app.rs`)

- After token entry, add a new `SetupStep::NotificationMode` screen that lets the user pick
  Telegram, LAN, or Both.
- For LAN mode, optionally prompt for the broadcast address (default `255.255.255.255:5005`).
- Persist the choice into `AppConfig`.

#### 4. Dispatch the right notifier at runtime (`src/tui/app.rs`)

In `tick_kstars_monitor()`, build the correct notifier based on `config.notification_mode`:

```rust
let notifier: Box<dyn Notification> = match config.notification_mode {
    NotificationMode::Telegram => Box::new(TelegramNotifier::new(&config.token)),
    NotificationMode::Lan      => Box::new(LanNotifier::new(&config.lan_broadcast_addr)),
    NotificationMode::Both     => Box::new(MultiNotifier::new(vec![...])),
};
notifier.send("KStars has stopped.")?;
```

A `MultiNotifier` wrapper (also implementing `Notification`) can fan out to multiple backends.

#### 5. Document the LAN listener side

Add a section to `README.md` explaining how a user can run a simple UDP listener on their
phone or secondary computer to receive the datagram and show a local notification
(e.g. using `netcat`, a Python snippet, or a future companion app).

---

### Out of scope (for now)

- A full companion mobile app.
- MQTT broker setup (suggested in the issue for Android — too heavy a dependency for v1).
- mDNS/Bonjour auto-discovery of listener devices.
