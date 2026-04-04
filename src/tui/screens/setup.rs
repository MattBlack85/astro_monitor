use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::tui::app::{App, AppState, SetupStep};

pub fn render_setup(f: &mut Frame, app: &App) {
    match &app.state {
        AppState::Setup(step) => match step {
            SetupStep::Instructions => render_instructions(f),
            SetupStep::TokenEntry => render_token_entry(f, app),
            SetupStep::Confirm => render_confirm(f, app),
        },
        _ => {}
    }
}

/// Returns a horizontally and vertically centered Rect of the given width% and fixed height.
fn centered_rect(area: Rect, width_pct: u16, height: u16) -> Rect {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100u16.saturating_sub(height * 100 / area.height.max(1))) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let h = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_pct) / 2),
            Constraint::Percentage(width_pct),
            Constraint::Min(0),
        ])
        .split(v[1]);

    h[1]
}

fn render_instructions(f: &mut Frame) {
    let area = f.size();
    let panel = centered_rect(area, 70, 16);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  No configuration found.",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  To use AstroMonitor you need a token from"),
        Line::from("  the Telegram bot."),
        Line::from(""),
        Line::from("  Steps to get your token:"),
        Line::from(""),
        Line::from("    1. Open Telegram"),
        Line::from("    2. Search for @AstroMonitorBot"),
        Line::from("    3. Send /register"),
        Line::from("    4. Copy the token you receive"),
        Line::from(""),
        Line::from(Span::styled(
            "  [Press Enter to continue]",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ];

    let block = Block::default()
        .title(" AstroMonitor — First-Time Setup ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, panel);
    f.render_widget(Paragraph::new(text).block(block), panel);
}

fn render_token_entry(f: &mut Frame, app: &App) {
    let area = f.size();
    let panel = centered_rect(area, 60, 7);

    let input_line = format!(" {} ", app.token_input);

    let text = vec![
        Line::from(""),
        Line::from("  Enter your token:"),
        Line::from(""),
        Line::from(Span::styled(
            input_line,
            Style::default().fg(Color::White).bg(Color::DarkGray),
        )),
        Line::from(""),
    ];

    let block = Block::default()
        .title(" Token Entry ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, panel);
    f.render_widget(Paragraph::new(text).block(block), panel);

    // Hint line below the panel
    let hint_y = panel.y + panel.height;
    if hint_y < area.height {
        let hint_area = Rect {
            x: panel.x,
            y: hint_y,
            width: panel.width,
            height: 1,
        };
        let hint = Paragraph::new(Line::from(vec![
            Span::styled("  [Enter] Confirm  ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Esc] Back", Style::default().fg(Color::DarkGray)),
        ]));
        f.render_widget(hint, hint_area);
    }
}

fn render_confirm(f: &mut Frame, app: &App) {
    let area = f.size();
    let panel = centered_rect(area, 60, 9);

    let masked = mask_token(&app.token_input);

    let confirm_style = if app.confirm_focus == 0 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let cancel_style = if app.confirm_focus == 1 {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Red)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Red)
    };

    let text = vec![
        Line::from(""),
        Line::from("  Token to save:"),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", masked),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("    "),
            Span::styled("[ Confirm ]", confirm_style),
            Span::raw("   "),
            Span::styled("[ Cancel ]", cancel_style),
        ]),
        Line::from(""),
    ];

    let block = Block::default()
        .title(" Confirm Token ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(Clear, panel);
    f.render_widget(Paragraph::new(text).block(block), panel);

    // Hint line below the panel
    let hint_y = panel.y + panel.height;
    if hint_y < area.height {
        let hint_area = Rect {
            x: panel.x,
            y: hint_y,
            width: panel.width,
            height: 1,
        };
        let hint = Paragraph::new(Line::from(vec![
            Span::styled(
                "  [←→/Tab] Switch  ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("[Enter] Activate  ", Style::default().fg(Color::DarkGray)),
            Span::styled("[Esc] Back", Style::default().fg(Color::DarkGray)),
        ]));
        f.render_widget(hint, hint_area);
    }
}

fn mask_token(token: &str) -> String {
    let len = token.len();
    if len == 0 {
        String::new()
    } else if len <= 8 {
        "*".repeat(len)
    } else {
        format!("{}****{}", &token[..4], &token[len - 4..])
    }
}
