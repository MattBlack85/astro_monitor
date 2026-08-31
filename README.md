# Astro monitor
<p align="center">
  <img src="https://github.com/MattBlack85/astro_monitor/assets/4163222/5798898a-2569-49e3-b60c-f783b9134bf6" alt="Your image title" width="200"/>
</p>


A small program that can help you with your astro session.
AstroMonitor can backup your astronomy configuration files and restore them on any machine:
- INDI device profiles
- KStars equipment profile and user database
- PHD2 guide profile
- KStars settings (theme, colors, etc.)

You can restore the backup on another PC or after a fresh install.

# Install
Trust me? Run the following command:

```shell
wget -O - https://raw.githubusercontent.com/MattBlack85/astro_monitor/main/install.sh | sh
```

`sudo` is needed at the last step to move `astromonitor` to `/usr/local/bin`.

# How to use AstroMonitor

AstroMonitor 2.0 features a fully interactive Terminal UI — no flags needed.

## First launch (no config found)

1. Run `astromonitor` in a terminal.
2. The setup wizard appears automatically.
3. Follow the on-screen instructions: open Telegram, search for `@AstroMonitorBot`, send `/register`, and copy the token you receive.
4. Enter the token when prompted.
5. Choose where alerts should be delivered — Telegram, LAN, or both (see below).
6. Confirm. Your settings are saved to `~/.config/astromonitor/astro.json` — you won't need to enter them again.

## Notifications without internet

The Telegram bot relays through a server on the internet. At a dark site
there usually isn't any, so an alert about a session that has gone wrong
is lost exactly when it matters most.

Choosing **LAN** (or **Both**) makes AstroMonitor send the alert as a UDP
datagram on the local network instead. No relay, no broker, no internet —
just a listener on the same machine or the same network that surfaces the
message. **Both** is the safe default for a machine that is sometimes
online: the alert counts as delivered if either route gets through.

The datagram payload is the message itself, as plain text, so the
simplest possible listener works:

```shell
nc -ul 5005
```

The destination is configurable during setup:

| Destination | When to use it |
|---|---|
| `255.255.255.255:5005` (default) | a listener on another device on the same network |
| `127.0.0.1:5005` | a listener on this same machine |

If you run [astroarch-bridge](https://github.com/Johannes1979I/astroarch-bridge)
it can act as that listener: it republishes what it receives to every
connected client, so the alert reaches your phone, tablet or laptop
browser with nothing else to install. Use `127.0.0.1:5005` when the
bridge runs on this machine, which is the usual case; to receive from
another host, point AstroMonitor at the broadcast address and set
`ASTROARCH_NOTIFY_UDP_HOST=0.0.0.0` on the bridge.

Existing configuration files keep working: without an explicit choice,
notifications stay on Telegram exactly as before.

## Dashboard

After setup (or on every subsequent launch) you land on the main dashboard:

```
┌─────────────────────────────────────────────────┐
│           AstroMonitor 2.0                      │
├─────────────────────────────────────────────────┤
│                                                 │
│      ┌─────────────────┐                        │
│      │   Take Backup   │  ← focused             │
│      └─────────────────┘                        │
│                                                 │
│      ┌─────────────────┐                        │
│      │ Restore Backup  │                        │
│      └─────────────────┘                        │
│                                                 │
├─────────────────────────────────────────────────┤
│  [↑↓] Navigate   [Enter] Select   [q] Quit      │
└─────────────────────────────────────────────────┘
```

- **↑ / ↓** — move focus between buttons
- **Enter** — run the selected operation
- **q** or **Esc** — quit

A status bar shows the result of each operation (success or error).

# Compile it yourself

The project is pure Rust. Install the toolchain from [rustup.rs](https://rustup.rs/), then:

```shell
git clone https://github.com/MattBlack85/astro_monitor
cd astro_monitor
cargo build --release
```

The compiled binary is at `target/release/astromonitor`. Move it wherever you like (e.g. `/usr/local/bin`).
