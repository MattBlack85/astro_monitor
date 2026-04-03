# CLAUDE.md - AI Assistant Guide for astro_monitor

## Project Overview

**astromonitor** is a Rust-based CLI watchdog tool for amateur astronomers using [KStars](https://kstars.kde.org/) astronomy software. It monitors the KStars process, sends crash notifications via Telegram, logs system resource usage, and manages backup/restore of astronomy configuration files.

- **Language:** Rust (Edition 2021)
- **Binary name:** `astromonitor`
- **Version:** 1.1.0
- **Supported platforms:** Linux (AMD64, ARM64/Raspberry Pi), macOS (Intel, Apple Silicon)

---

## Repository Structure

```
astro_monitor/
├── .github/workflows/build-check.yml   # CI/CD: multi-target build and release
├── src/
│   ├── main.rs                         # CLI entry point, orchestration, Telegram notify
│   ├── lib.rs                          # Core types: CliArgs, Paths, constants
│   ├── backup/
│   │   ├── mod.rs                      # Module re-exports
│   │   └── database.rs                 # Backup upload and restore logic
│   ├── checks/
│   │   ├── mod.rs                      # Module re-exports
│   │   ├── system.rs                   # lsof detection
│   │   └── vault.rs                    # Log directory initialization
│   └── monitoring/
│       ├── mod.rs                      # Module re-exports
│       └── resources.rs                # CPU/RAM logging
├── Cargo.toml                          # Rust project manifest and dependencies
├── Cargo.lock                          # Locked dependency versions
├── install.sh                          # End-user install script
└── README.md                           # End-user documentation
```

---

## Key Architecture

### CLI Structure (`src/lib.rs`, `src/main.rs`)

The `CliArgs` struct (via `structopt`) defines all CLI flags:
- `--kstars <TOKEN>` — monitor KStars process; send Telegram alert on crash
- `--do-backup <TOKEN>` — create TAR archive of astronomy configs and upload to server
- `--retrieve-backup <TOKEN>` — download and restore backed-up configs
- `--fd-monitor` — log open file descriptors (requires `lsof` on PATH)
- `--system-monitor` — periodically log CPU/RAM usage to timestamped files

All features requiring a token use a Telegram bot (`@AstroMonitorBot`) for authentication and notifications.

### Key Constants (`src/lib.rs`)
- `INTERVAL = 15` — polling interval in seconds for process/resource monitoring
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
cargo run -- --help
cargo run -- --kstars <TOKEN>
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
ASTROMONITOR_LOG_LEVEL=debug cargo run -- --system-monitor
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
| `structopt` | 0.3 | CLI argument parsing |
| `sysinfo` | 0.27 | CPU, RAM, process info |
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
4. **The polling loop in `main.rs`** uses `std::thread::sleep(Duration::from_secs(INTERVAL))`. Changes to monitoring logic should preserve this pattern.
5. **No async runtime** — the project is fully synchronous. Do not introduce `tokio` or `async-std` without strong justification.
6. **`Cargo.lock` is committed** — this is a binary application, so the lock file should stay tracked in git.
7. **CI targets ARM** — avoid `x86_64`-only APIs or system calls; test cross-compilation when adding system-level features.
8. **`structopt` is legacy** — it wraps `clap` v2. New CLI projects should use `clap` v4 directly, but do not migrate this unless asked.
