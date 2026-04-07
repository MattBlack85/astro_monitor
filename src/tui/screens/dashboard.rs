use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::tui::app::App;

const BUTTONS: [&str; 3] = ["Take Backup", "Restore Backup", "Watch KStars"];

pub fn render_dashboard(f: &mut Frame, app: &App) {
    let area = f.area();

    // Outer block with title
    let outer_block = Block::default()
        .title(" AstroMonitor 2.0 ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(outer_block, area);

    // Inner area split: top (buttons) / bottom (hints + status)
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // spacer / buttons area
            Constraint::Length(1), // status bar
            Constraint::Length(1), // hint bar
        ])
        .split(inner);

    // --- Buttons ---
    render_buttons(f, app, chunks[0]);

    // --- Status bar ---
    let status_text = match &app.status_message {
        Some((msg, true)) => Line::from(Span::styled(
            format!(" {}", msg),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Some((msg, false)) => Line::from(Span::styled(
            format!(" {}", msg),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        None => Line::from(""),
    };
    f.render_widget(Paragraph::new(status_text), chunks[1]);

    // --- Hint bar ---
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" [↑↓] Navigate", Style::default().fg(Color::DarkGray)),
        Span::styled("   [Enter] Select", Style::default().fg(Color::DarkGray)),
        Span::styled("   [q] Quit", Style::default().fg(Color::DarkGray)),
    ]));
    f.render_widget(hints, chunks[2]);
}

fn render_buttons(f: &mut Frame, app: &App, area: Rect) {
    // Stack buttons vertically in the center of the available area
    let button_height: u16 = 3;
    let gap: u16 = 1;
    let total_height = BUTTONS.len() as u16 * button_height + (BUTTONS.len() as u16 - 1) * gap;
    let button_width: u16 = 25;

    let top_pad = area.height.saturating_sub(total_height) / 2;
    let left_pad = area.width.saturating_sub(button_width) / 2;

    for (i, label) in BUTTONS.iter().enumerate() {
        let y = area.y + top_pad + i as u16 * (button_height + gap);
        if y + button_height > area.y + area.height {
            break;
        }
        let btn_area = Rect {
            x: area.x + left_pad,
            y,
            width: button_width,
            height: button_height,
        };

        let focused = app.dashboard_focus == i;
        let (border_style, text_style) = if focused {
            (
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            (
                Style::default().fg(Color::White),
                Style::default().fg(Color::White),
            )
        };

        let block = Block::default().borders(Borders::ALL).style(border_style);
        f.render_widget(Clear, btn_area);
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                format!("  {}  ", label),
                text_style,
            )))
            .block(block)
            .alignment(Alignment::Center),
            btn_area,
        );
    }
}
