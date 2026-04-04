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
4. Enter the token when prompted, then confirm.
5. Your token is saved to `~/.config/astromonitor/astro.json` — you won't need to enter it again.

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
