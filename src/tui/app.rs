use astromonitor::config::{AppConfig, load_config, save_config};
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
    pub confirm_focus: usize, // 0 = Confirm button, 1 = Cancel button
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
            // Phase 4 — placeholder quit handling
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                self.running = false;
            }
        }
    }
}
