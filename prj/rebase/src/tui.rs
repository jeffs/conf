//! Ratatui front end that runs repos through a bounded worker pool.

use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, Event as TermEvent, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use crate::jj::Mode;
use crate::manifest::Repo;
use crate::ops::{Op, Outcome, Runner};
use crate::output::{Kind, Sink};

/// Progress reported by worker threads.
enum Event {
    Started(usize),
    Line(usize, Kind, String),
    Done(usize, Outcome),
}

enum State {
    Pending,
    Running,
    Done(Outcome),
}

struct RepoTask {
    name: String,
    state: State,
    lines: Vec<(Kind, String)>,
}

/// Sends one repo's progress lines into the shared event channel.
struct ChannelSink {
    repo: usize,
    tx: mpsc::Sender<Event>,
}

impl Sink for ChannelSink {
    fn line(&self, kind: Kind, text: String) {
        let _ = self.tx.send(Event::Line(self.repo, kind, text));
    }
}

/// Run `op` over `repos`, at most `jobs` at a time, rendering progress.
/// Returns whether every repo finished ok (or was skipped).
///
/// # Errors
///
/// Returns an error if the terminal cannot be drawn or read.
pub fn run(op: Op, repos: &[Repo], jobs: usize, mode: Mode) -> io::Result<bool> {
    let mut tasks: Vec<RepoTask> = repos
        .iter()
        .map(|r| RepoTask {
            name: r.name.clone(),
            state: State::Pending,
            lines: Vec::new(),
        })
        .collect();

    let rx = spawn_workers(op, repos.to_vec(), jobs, mode);

    let mut terminal = ratatui::init();
    let result = event_loop(&mut terminal, &mut tasks, &rx);
    ratatui::restore();
    result
}

/// Start up to `jobs` worker threads that pull repos off a shared queue.
fn spawn_workers(op: Op, repos: Vec<Repo>, jobs: usize, mode: Mode) -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel();
    let repos = Arc::new(repos);
    let next = Arc::new(AtomicUsize::new(0));
    for _ in 0..jobs.clamp(1, repos.len().max(1)) {
        let repos = Arc::clone(&repos);
        let next = Arc::clone(&next);
        let tx = tx.clone();
        thread::spawn(move || {
            loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(repo) = repos.get(i) else { break };
                let _ = tx.send(Event::Started(i));
                let sink = ChannelSink {
                    repo: i,
                    tx: tx.clone(),
                };
                let runner = Runner { mode, sink: &sink };
                let _ = tx.send(Event::Done(i, runner.run_one(op, repo)));
            }
        });
    }
    rx
}

fn event_loop(
    terminal: &mut DefaultTerminal,
    tasks: &mut [RepoTask],
    rx: &mpsc::Receiver<Event>,
) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| render(frame, tasks))?;

        while let Ok(ev) = rx.try_recv() {
            apply(tasks, ev);
        }

        if event::poll(Duration::from_millis(50))?
            && let TermEvent::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            return Ok(false);
        }

        if tasks.iter().all(|t| matches!(t.state, State::Done(_))) {
            break;
        }
    }

    terminal.draw(|frame| render(frame, tasks))?;
    let all_ok = tasks
        .iter()
        .all(|t| matches!(&t.state, State::Done(Outcome::Ok | Outcome::Skipped(_))));
    if all_ok {
        thread::sleep(Duration::from_millis(500));
    } else {
        wait_for_quit()?;
    }
    Ok(all_ok)
}

fn wait_for_quit() -> io::Result<()> {
    loop {
        if let TermEvent::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            return Ok(());
        }
    }
}

fn apply(tasks: &mut [RepoTask], event: Event) {
    match event {
        Event::Started(i) => tasks[i].state = State::Running,
        Event::Line(i, kind, text) => tasks[i].lines.push((kind, text)),
        Event::Done(i, outcome) => tasks[i].state = State::Done(outcome),
    }
}

fn render(frame: &mut Frame, tasks: &[RepoTask]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(frame.area());

    render_tasks(frame, chunks[0], tasks);
    render_status_bar(frame, chunks[1], tasks);
}

fn render_tasks(frame: &mut Frame, area: Rect, tasks: &[RepoTask]) {
    let block = Block::default().title("Repos").borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(2); tasks.len()])
        .split(inner);

    for (task, task_area) in tasks.iter().zip(areas.iter()) {
        render_task(frame, *task_area, task);
    }
}

fn render_task(frame: &mut Frame, area: Rect, task: &RepoTask) {
    let style = header_style(&task.state);
    let mut header = vec![
        Span::styled(format!("{} ", icon(&task.state)), style),
        Span::styled(task.name.as_str(), style),
    ];
    if let State::Done(Outcome::Skipped(reason) | Outcome::Failed(reason)) = &task.state {
        header.push(Span::styled(format!(" — {reason}"), style));
    }

    let mut lines = vec![Line::from(header)];
    let visible = usize::from(area.height).saturating_sub(1);
    let start = task.lines.len().saturating_sub(visible);
    for (kind, text) in task.lines.iter().skip(start) {
        let text = match kind {
            Kind::Cmd => format!("  $ {text}"),
            Kind::DryRun => format!("  [dry-run] {text}"),
            _ => format!("  {text}"),
        };
        lines.push(Line::styled(text, line_style(*kind)));
    }
    frame.render_widget(Paragraph::new(lines), area);
}

fn header_style(state: &State) -> Style {
    match state {
        State::Done(Outcome::Ok) => Style::default().fg(Color::Green),
        State::Done(Outcome::Skipped(_)) => Style::default().fg(Color::Yellow),
        State::Done(Outcome::Failed(_)) => Style::default().fg(Color::Red),
        State::Running => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        State::Pending => Style::default().fg(Color::DarkGray),
    }
}

const fn icon(state: &State) -> &'static str {
    match state {
        State::Pending => "○",
        State::Running => "◐",
        State::Done(Outcome::Ok) => "✓",
        State::Done(Outcome::Skipped(_)) => "−",
        State::Done(Outcome::Failed(_)) => "✗",
    }
}

fn line_style(kind: Kind) -> Style {
    match kind {
        Kind::Ok => Style::default().fg(Color::Green),
        Kind::Warn => Style::default().fg(Color::Yellow),
        Kind::Error => Style::default().fg(Color::Red),
        Kind::Cmd | Kind::DryRun | Kind::Info | Kind::Out => Style::default().fg(Color::DarkGray),
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, tasks: &[RepoTask]) {
    let done = tasks
        .iter()
        .filter(|t| matches!(t.state, State::Done(_)))
        .count();
    let failed = tasks
        .iter()
        .filter(|t| matches!(t.state, State::Done(Outcome::Failed(_))))
        .count();
    let total = tasks.len();

    let status = if failed > 0 {
        format!("{done}/{total} done, {failed} failed │ q to quit")
    } else {
        format!("{done}/{total} done │ q to quit")
    };

    let block = Block::default().borders(Borders::TOP);
    frame.render_widget(Paragraph::new(status).block(block), area);
}
