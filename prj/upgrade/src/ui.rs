use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::task::{Section, State, Task};

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

    // Group tasks by section, preserving order.
    let sections: Vec<Section> = {
        let mut seen = std::collections::HashSet::new();
        tasks
            .iter()
            .filter_map(|t| seen.insert(t.section).then_some(t.section))
            .collect()
    };

    // Build constraints: 1 line per section header, Min(3) per task.
    let mut constraints = Vec::new();
    for section in &sections {
        constraints.push(Constraint::Length(1)); // Section header
        for task in tasks.iter().filter(|t| t.section == *section) {
            let _ = task;
            constraints.push(Constraint::Min(3));
        }
    }

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    let mut area_idx = 0;
    for section in &sections {
        render_section_header(frame, areas[area_idx], *section);
        area_idx += 1;

        for task in tasks.iter().filter(|t| t.section == *section) {
            render_task(frame, areas[area_idx], task);
            area_idx += 1;
        }
    }
}

fn render_section_header(frame: &mut Frame, area: Rect, section: Section) {
    let header = Line::from(Span::styled(
        section.label(),
        Style::default().add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(Paragraph::new(header), area);
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
