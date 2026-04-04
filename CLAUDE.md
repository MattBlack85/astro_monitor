# CLAUDE.md - AI Assistant Guide for astro_monitor

## Project Overview

**astromonitor** is a Rust-based TUI application for amateur astronomers using [KStars](https://kstars.kde.org/) astronomy software. It provides an interactive terminal dashboard to back up and restore astronomy configuration files, with Telegram integration for notifications.

- **Language:** Rust (Edition 2021)
- **Binary name:** `astromonitor`
- **Version:** 2.0.0
- **Supported platforms:** Linux (AMD64, ARM64/Raspberry Pi), macOS (Intel, Apple Silicon)

---

## Repository Structure

```
astro_monitor/
├── .github/workflows/build-check.yml   # CI/CD: multi-target build and release
├── src/
│   ├── main.rs                         # Entry point: calls tui::runner::run()
│   ├── lib.rs                          # Core types: Paths, HOST constant
│   ├── config.rs                       # AppConfig load/save (astro.json)
│   ├── backup/
│   │   ├── mod.rs                      # Module re-exports
│   │   └── database.rs                 # Backup upload and restore logic
│   ├── checks/
│   │   ├── mod.rs                      # Module re-exports
│   │   ├── system.rs                   # lsof detection (reserved for future TUI use)
│   │   └── vault.rs                    # Log directory initialization
│   ├── monitoring/
│   │   ├── mod.rs                      # Module re-exports
│   │   ├── resources.rs                # CPU/RAM logging (reserved for future TUI use)
│   │   └── telegram.rs                 # Telegram notification helper
│   └── tui/
│       ├── mod.rs                      # TUI module root
│       ├── app.rs                      # App struct, AppState enum, input handling
│       ├── runner.rs                   # Terminal init, main event loop
│       ├── ui.rs                       # Top-level render dispatcher
│       └── screens/
│           ├── mod.rs                  # Screen module re-exports
│           ├── setup.rs                # First-run setup wizard screens
│           ├── dashboard.rs            # Main action dashboard
│           └── feedback.rs            # Working / Result screens
├── Cargo.toml                          # Rust project manifest and dependencies
├── Cargo.lock                          # Locked dependency versions
├── install.sh                          # End-user install script
└── README.md                           # End-user documentation
```

---

## Key Architecture

### TUI Structure (`src/tui/`)

The application is fully driven by a `ratatui`/`crossterm` Terminal UI. There are no CLI flags.

**Application states (`src/tui/app.rs`)**:

```
AppState::Boot
    │
    ├─ config exists ──► AppState::Dashboard
    │
    └─ config missing ──► AppState::Setup(SetupStep::Instructions)
                               │
                               └─► AppState::Setup(SetupStep::TokenEntry)
                                        │
                                        └─► AppState::Setup(SetupStep::Confirm)
                                                 │
                                                 ├─ confirmed ──► (save) ──► AppState::Dashboard
                                                 └─ cancelled ──► AppState::Setup(SetupStep::TokenEntry)

AppState::Dashboard ──► AppState::Working { label } ──► AppState::Result { message, success }
                                                               │
                                                               └─► AppState::Dashboard (any key)
```

- `App::new()` loads the config to determine initial state.
- `App::handle_key()` dispatches key events to the active state handler.
- `App::execute_pending_op()` runs the backup/restore synchronously after the Working frame is drawn.

### Config File (`src/config.rs`)

**Path:** `~/.config/astromonitor/astro.json`

```json
{
  "token": "<telegram bot token>"
}
```

- `load_config() -> Option<AppConfig>` — reads and deserializes; returns `None` on missing/malformed file
- `save_config(config: &AppConfig) -> Result<(), String>` — creates directory if needed, writes JSON

### Key Constants (`src/lib.rs`)
- `HOST = "http://astromatto.com:11111"` — hardcoded backup/notification server

### Cross-platform Paths (`src/lib.rs` — `Paths` struct)
Uses the `dirs` crate for platform-appropriate paths:
- **Linux:** `~/.local/share/astromonitor/` and `~/.config/`
- **macOS:** `~/Library/Application Support/astromonitor/` and `~/Library/Preferences/`

### Files Managed by Backup (`src/backup/database.rs`)
| File | Purpose |
|------|---------|
| `~/.local/share/kstars/userdb.sqlite` | KStars user database |
| `~/.local/share/kstars/mycitydb.sqlite` | City/location database |
| `~/.local/share/kstars/fov.dat` | Field of view data |
| `~/.config/kstarsrc` | KStars application config |
| `~/.indi/*.xml` | INDI device profiles |
| `~/.PHDGuidingV2` | PHD2 guide profile |

(macOS paths handled via conditional `#[cfg(target_os)]` in `Paths`)

---

## Development Workflow

### Building

```bash
# Debug build (fast, for development)
cargo build

# Release build (optimized, matches what CI produces)
cargo build --release

# Run directly
cargo run
```

### Cross-compilation (matches CI targets)

```bash
# Add a target
rustup target add aarch64-unknown-linux-gnu

# Install cross-linker (Linux host only)
sudo apt-get install gcc-aarch64-linux-gnu

# Configure in ~/.cargo/config
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

# Build for target
cargo build --release --target aarch64-unknown-linux-gnu
```

### Logging

The `env_logger` crate reads the `ASTROMONITOR_LOG_LEVEL` environment variable:
```bash
ASTROMONITOR_LOG_LEVEL=debug cargo run
```
Default level is `info`. Use `error`, `warn`, `info`, `debug`, or `trace`.

Logs are written to: `~/.local/share/astromonitor/logs/<timestamp>.log`

---

## CI/CD Pipeline (`.github/workflows/build-check.yml`)

### Triggers
- Push to `main` (excluding README.md and install.sh changes)
- Pull requests targeting `main`
- Tags matching `v*` (triggers release artifact upload)

### Build Matrix
| Target | Platform |
|--------|---------|
| `x86_64-unknown-linux-gnu` | Linux AMD64 |
| `aarch64-unknown-linux-gnu` | Linux ARM64 / Raspberry Pi |
| `x86_64-apple-darwin` | macOS Intel |
| `aarch64-apple-darwin` | macOS Apple Silicon |

### Release Artifacts
On `v*` tags, binaries are compressed as `astromonitor-{platform}-{tag}.tar.gz` and uploaded to GitHub Releases. The `install.sh` script downloads these.

---

## Code Conventions

### Naming
- Functions and variables: `snake_case`
- Structs and enums: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`

### Error Handling
- `panic!()` for unrecoverable failures (e.g., missing config directories during backup)
- `Result<T, String>` for recoverable errors returned from functions
- `match` / `if let` for unwrapping `Result`/`Option`

### Logging macros (from `log` crate)
- `error!()` — fatal/non-recoverable issues
- `warn!()` — unexpected but handled conditions
- `info!()` — normal operational events

### Module Layout
Each feature area has its own directory with a `mod.rs` that re-exports public items. Add new features as new modules or extend existing ones — avoid growing `main.rs`.

---

## Testing

There are currently **no automated tests** in this repository. There is no `tests/` directory and no `#[test]` functions in source files.

When adding tests:
- Unit tests go in the same file as the code under test, inside `#[cfg(test)] mod tests { ... }`
- Integration tests go in a top-level `tests/` directory
- Run tests with `cargo test`

---

## External Dependencies Summary

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.26 | Terminal UI framework |
| `crossterm` | 0.27 | Cross-platform terminal backend and input events |
| `serde` | 1 | Serialization/deserialization (derive macros) |
| `serde_json` | 1 | JSON config file format |
| `sysinfo` | 0.27 | CPU, RAM, process info (reserved for future TUI panels) |
| `minreq` | 2.6 | HTTP client (with TLS for Telegram/backup API) |
| `tar` | 0.4 | TAR archive creation and extraction |
| `chrono` | 0.4 | Timestamp formatting for log files |
| `dirs` | 4.0 | Cross-platform home/config directory paths |
| `log` | 0.4 | Logging macros |
| `env_logger` | 0.10 | Log level control via environment variable |

---

## Key External APIs

### Telegram Notification
- **Endpoint:** `POST http://astromatto.com:11111/hook/{token}`
- **Body:** Plain text message string
- **Purpose:** Alert user when KStars process stops

### Backup Upload
- **Endpoint:** `POST http://astromatto.com:11111/backup/db/{token}`
- **Content-Type:** `application/octet-stream`
- **Body:** Raw TAR archive bytes

### Backup Download
- **Endpoint:** `GET http://astromatto.com:11111/backup/db/{token}`
- **Response:** Raw TAR archive bytes, written to a temp file then extracted

---

## Important Notes for AI Assistants

1. **No test suite exists** — changes cannot be validated by running tests. Reason carefully about correctness.
2. **The backup server URL is hardcoded** — `http://astromatto.com:11111`. Any change here affects all users.
3. **Cross-platform paths are critical** — always use the `Paths` struct rather than hardcoded paths to ensure Linux/macOS compatibility.
4. **No CLI flags** — version 2.0 is entirely TUI-driven. Do not re-introduce `structopt` or `clap` unless asked.
5. **No async runtime** — the project is fully synchronous. Do not introduce `tokio` or `async-std` without strong justification.
6. **`Cargo.lock` is committed** — this is a binary application, so the lock file should stay tracked in git.
7. **CI targets ARM** — avoid `x86_64`-only APIs or system calls; test cross-compilation when adding system-level features.
8. **`checks/` and `monitoring/` modules are preserved** — they are kept for potential future TUI panels (e.g. system resource monitor, fd monitor) but are not wired into the TUI yet.
