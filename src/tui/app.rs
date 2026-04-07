use crate::backup::database::{retrieve_db, send_db};
use crate::checks::system::kstars_is_running;
use crate::notifications::Notification;
use crate::notifications::telegram::TelegramNotifier;
use astromonitor::Paths;
use astromonitor::config::{AppConfig, load_config, save_config};
use chrono::SecondsFormat;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub enum SetupStep {
    Instructions,
    TokenEntry,
    Confirm,
}

pub enum DashboardOp {
    TakeBackup,
    RestoreBackup,
}

pub enum AppState {
    Boot,
    Setup(SetupStep),
    Dashboard,
    Working {
        label: String,
    },
    Result {
        message: String,
        success: bool,
    },
    KStarsMonitor {
        started_at: String,
        last_seen_at: String,
    },
}

pub struct App {
    pub state: AppState,
    pub token_input: String,
    pub config: Option<AppConfig>,
    pub confirm_focus: usize,   // 0 = Confirm button, 1 = Cancel button
    pub dashboard_focus: usize, // 0 = Take Backup, 1 = Restore Backup
    pub status_message: Option<(String, bool)>, // (message, is_success)
    pub pending_op: Option<DashboardOp>,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        let config = load_config();
        let state = if config.is_some() {
            AppState::Dashboard
        } else {
            AppState::Setup(SetupStep::Instructions)
        };
        Self {
            state,
            token_input: String::new(),
            config,
            confirm_focus: 0,
            dashboard_focus: 0,
            status_message: None,
            pending_op: None,
            running: true,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Ignore key release and repeat events
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Global: Ctrl+C always quits
        if key.code == KeyCode::Char('c')
            && key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL)
        {
            self.running = false;
            return;
        }

        if matches!(self.state, AppState::Setup(SetupStep::Instructions)) {
            if key.code == KeyCode::Enter {
                self.state = AppState::Setup(SetupStep::TokenEntry);
            }
            return;
        }

        if matches!(self.state, AppState::Setup(SetupStep::TokenEntry)) {
            match key.code {
                KeyCode::Enter => {
                    if !self.token_input.is_empty() {
                        self.confirm_focus = 0;
                        self.state = AppState::Setup(SetupStep::Confirm);
                    }
                }
                KeyCode::Esc => {
                    self.state = AppState::Setup(SetupStep::Instructions);
                }
                KeyCode::Backspace => {
                    self.token_input.pop();
                }
                KeyCode::Char(c) => {
                    self.token_input.push(c);
                }
                _ => {}
            }
            return;
        }

        if matches!(self.state, AppState::Setup(SetupStep::Confirm)) {
            match key.code {
                KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                    self.confirm_focus = 1 - self.confirm_focus;
                }
                KeyCode::Enter => {
                    if self.confirm_focus == 0 {
                        let config = AppConfig {
                            token: self.token_input.clone(),
                        };
                        if save_config(&config).is_ok() {
                            self.config = Some(config);
                            self.state = AppState::Dashboard;
                        }
                    } else {
                        self.confirm_focus = 0;
                        self.state = AppState::Setup(SetupStep::TokenEntry);
                    }
                }
                KeyCode::Esc => {
                    self.confirm_focus = 0;
                    self.state = AppState::Setup(SetupStep::TokenEntry);
                }
                _ => {}
            }
            return;
        }

        if matches!(self.state, AppState::Dashboard) {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    self.running = false;
                }
                KeyCode::Up => {
                    if self.dashboard_focus > 0 {
                        self.dashboard_focus -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.dashboard_focus < 2 {
                        self.dashboard_focus += 1;
                    }
                }
                KeyCode::Enter => match self.dashboard_focus {
                    0 => {
                        self.pending_op = Some(DashboardOp::TakeBackup);
                        self.state = AppState::Working {
                            label: "Taking backup…".to_string(),
                        };
                    }
                    1 => {
                        self.pending_op = Some(DashboardOp::RestoreBackup);
                        self.state = AppState::Working {
                            label: "Restoring backup…".to_string(),
                        };
                    }
                    2 => self.start_kstars_monitor(),
                    _ => {}
                },
                _ => {}
            }
            return;
        }

        if matches!(self.state, AppState::KStarsMonitor { .. }) {
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                self.state = AppState::Dashboard;
            }
            return;
        }

        if matches!(self.state, AppState::Result { .. }) {
            self.state = AppState::Dashboard;
        }
    }

    /// Check if KStars is running and enter KStarsMonitor state, or show an error.
    fn start_kstars_monitor(&mut self) {
        if kstars_is_running() {
            let now = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
            self.state = AppState::KStarsMonitor {
                started_at: now.clone(),
                last_seen_at: now,
            };
        } else {
            self.state = AppState::Result {
                message: "KStars is not running. Please start KStars first.".to_string(),
                success: false,
            };
        }
    }

    /// Called on each watchdog tick. Checks the KStars process and notifies if it stopped.
    pub fn tick_kstars_monitor(&mut self) {
        if kstars_is_running() {
            let now = chrono::Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);
            if let AppState::KStarsMonitor {
                ref mut last_seen_at,
                ..
            } = self.state
            {
                *last_seen_at = now;
            }
        } else {
            let token = self
                .config
                .as_ref()
                .map(|c| c.token.clone())
                .unwrap_or_default();
            let notifier = TelegramNotifier::new(token);
            let message = match notifier.send("KStars has stopped.") {
                Ok(_) => "KStars stopped. Telegram notification sent.".to_string(),
                Err(e) => format!("KStars stopped. Notification failed: {}", e),
            };
            self.state = AppState::Result {
                message,
                success: true,
            };
        }
    }

    /// Execute the pending backup/restore operation and transition to Result.
    /// Called by the runner after the Working frame has been drawn.
    pub fn execute_pending_op(&mut self) {
        if let Some(op) = self.pending_op.take() {
            let token = self
                .config
                .as_ref()
                .map(|c| c.token.clone())
                .unwrap_or_default();
            let paths = Paths::init();
            let (result, success_msg) = match &op {
                DashboardOp::TakeBackup => {
                    (send_db(&paths, &token), "Backup completed successfully.")
                }
                DashboardOp::RestoreBackup => {
                    (retrieve_db(&paths, &token), "Backup restored successfully.")
                }
            };
            let (message, success) = match result {
                Ok(_) => (success_msg.to_string(), true),
                Err(e) => (format!("Error: {}", e), false),
            };
            self.state = AppState::Result { message, success };
        }
    }
}
