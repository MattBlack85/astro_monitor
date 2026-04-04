use crate::backup::database::{retrieve_db, send_db};
use astromonitor::config::{AppConfig, load_config, save_config};
use astromonitor::Paths;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub enum SetupStep {
    Instructions,
    TokenEntry,
    Confirm,
}

pub enum AppState {
    Boot,
    Setup(SetupStep),
    Dashboard,
}

pub struct App {
    pub state: AppState,
    pub token_input: String,
    pub config: Option<AppConfig>,
    pub confirm_focus: usize,    // 0 = Confirm button, 1 = Cancel button
    pub dashboard_focus: usize,  // 0 = Take Backup, 1 = Restore Backup
    pub status_message: Option<(String, bool)>, // (message, is_success)
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
            && key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
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
                    if self.dashboard_focus < 1 {
                        self.dashboard_focus += 1;
                    }
                }
                KeyCode::Enter => {
                    let token = self
                        .config
                        .as_ref()
                        .map(|c| c.token.clone())
                        .unwrap_or_default();
                    let paths = Paths::init();
                    let result = if self.dashboard_focus == 0 {
                        send_db(&paths, &token)
                    } else {
                        retrieve_db(&paths, &token)
                    };
                    self.status_message = Some(match result {
                        Ok(_) => {
                            let label = if self.dashboard_focus == 0 {
                                "Backup completed successfully."
                            } else {
                                "Backup restored successfully."
                            };
                            (label.to_string(), true)
                        }
                        Err(e) => (format!("Error: {}", e), false),
                    });
                }
                _ => {}
            }
        }
    }
}
