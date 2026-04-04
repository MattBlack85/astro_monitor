use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_working(f: &mut Frame, label: &str) {
    let area = f.size();

    let outer_block = Block::default()
        .title(" AstroMonitor 2.0 ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(outer_block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Percentage(45),
        ])
        .split(ratatui::layout::Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        });

    let spinner = Paragraph::new(Line::from(Span::styled(
        "⠿  Working…",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center);
    f.render_widget(spinner, inner[1]);

    let detail = Paragraph::new(Line::from(Span::styled(
        label,
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);
    f.render_widget(detail, inner[2]);
}

pub fn render_result(f: &mut Frame, message: &str, success: bool) {
    let area = f.size();

    let outer_block = Block::default()
        .title(" AstroMonitor 2.0 ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(outer_block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Percentage(40),
            Constraint::Length(1),
        ])
        .split(ratatui::layout::Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        });

    let color = if success { Color::Green } else { Color::Red };
    let icon = if success { "✓" } else { "✗" };

    let result_line = Paragraph::new(Line::from(Span::styled(
        format!("{}  {}", icon, message),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center);
    f.render_widget(result_line, inner[1]);

    let hint = Paragraph::new(Line::from(Span::styled(
        "Press any key to return to Dashboard",
        Style::default().fg(Color::DarkGray),
    )))
    .alignment(Alignment::Center);
    f.render_widget(hint, inner[2]);
}
