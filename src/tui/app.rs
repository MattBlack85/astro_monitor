use astromonitor::config::{AppConfig, load_config};
use crossterm::event::KeyEvent;

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
            running: true,
        }
    }

    pub fn handle_key(&mut self, _key: KeyEvent) {
        // Central input dispatcher — delegates to active screen handler.
        // Implemented in Phase 3 (setup screens) and Phase 4 (dashboard).
    }
}
