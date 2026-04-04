use ratatui::Frame;

use super::app::{App, AppState};
use super::screens::{dashboard, setup};

pub fn render(f: &mut Frame, app: &App) {
    match &app.state {
        AppState::Boot => {}
        AppState::Setup(_) => setup::render_setup(f, app),
        AppState::Dashboard => dashboard::render_dashboard(f, app),
    }
}
