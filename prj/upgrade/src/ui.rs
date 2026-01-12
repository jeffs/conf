use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::task::{State, Task};

pub fn render(frame: &mut Frame, tasks: &[Task]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.area());

    render_tasks(frame, chunks[0], tasks);
    render_status_bar(frame, chunks[1], tasks);
}

fn render_tasks(frame: &mut Frame, area: Rect, tasks: &[Task]) {
    let block = Block::default()
        .title("Tasks")
        .borders(Borders::ALL);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if tasks.is_empty() {
        return;
    }

    let constraints: Vec<_> = tasks.iter().map(|_| Constraint::Min(3)).collect();

    let task_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (task, area) in tasks.iter().zip(task_areas.iter()) {
        render_task(frame, *area, task);
    }
}

fn render_task(frame: &mut Frame, area: Rect, task: &Task) {
    let style = match &task.state {
        State::Completed => Style::default().fg(Color::Green),
        State::Failed(_) => Style::default().fg(Color::Red),
        State::Running => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        _ => Style::default().fg(Color::DarkGray),
    };

    let header = Line::from(vec![
        Span::styled(format!("{} ", task.state.icon()), style),
        Span::styled(task.label, style),
    ]);

    let mut lines = vec![header];

    let visible_lines = (area.height as usize).saturating_sub(1);
    let start = task.output.len().saturating_sub(visible_lines);
    for line in task.output.iter().skip(start) {
        lines.push(Line::from(format!("  {line}")));
    }

    if let State::Failed(ref msg) = task.state {
        lines.push(Line::styled(format!("  error: {msg}"), Style::default().fg(Color::Red)));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

fn render_status_bar(frame: &mut Frame, area: Rect, tasks: &[Task]) {
    let completed = tasks.iter().filter(|t| matches!(t.state, State::Completed)).count();
    let failed = tasks.iter().filter(|t| matches!(t.state, State::Failed(_))).count();
    let total = tasks.len();

    let status = if failed > 0 {
        format!("{completed}/{total} complete, {failed} failed │ Press q to quit")
    } else {
        format!("{completed}/{total} complete │ Press q to quit")
    };

    let block = Block::default().borders(Borders::TOP);
    let paragraph = Paragraph::new(status).block(block);
    frame.render_widget(paragraph, area);
}
