use std::error::Error;
use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use super::app::{App, AppState};
use super::ui;

/// How often the KStars watchdog checks the process list.
const WATCHDOG_POLL_INTERVAL: Duration = Duration::from_secs(5);

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let result = (|| -> Result<(), Box<dyn Error>> {
        while app.running {
            terminal.draw(|f| ui::render(f, &app))?;

            // If we are in the Working state, run the operation synchronously
            // (the Working frame has already been drawn above) then loop again
            // to draw the Result frame without consuming a key event.
            if matches!(app.state, AppState::Working { .. }) {
                app.execute_pending_op();
                continue;
            }

            // In KStarsMonitor state, use a timed poll so the watchdog can tick
            // periodically even when the user presses no keys.
            if matches!(app.state, AppState::KStarsMonitor { .. }) {
                if event::poll(WATCHDOG_POLL_INTERVAL)? {
                    if let Event::Key(key) = event::read()? {
                        app.handle_key(key);
                    }
                } else {
                    app.tick_kstars_monitor();
                }
                continue;
            }

            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }
        Ok(())
    })();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
