use ratatui::Frame;

use super::app::{App, AppState};
use super::screens::{dashboard, feedback, setup};

pub fn render(f: &mut Frame, app: &App) {
    match &app.state {
        AppState::Boot => {}
        AppState::Setup(_) => setup::render_setup(f, app),
        AppState::Dashboard => dashboard::render_dashboard(f, app),
        AppState::Working { label } => feedback::render_working(f, label),
        AppState::Result { message, success } => feedback::render_result(f, message, *success),
        AppState::KStarsMonitor { started_at, last_seen_at } => {
            feedback::render_kstars_monitor(f, started_at, last_seen_at)
        }
    }
}
