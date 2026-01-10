use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc;

use crate::runner;
use crate::task::{Lane, State, Task, tasks};
use crate::SshEnv;

pub struct App {
    tasks: Vec<Task>,
    rx: mpsc::UnboundedReceiver<runner::Event>,
    tx: mpsc::UnboundedSender<runner::Event>,
    ssh_env: SshEnv,
}

impl App {
    pub fn new(ssh_env: SshEnv) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            tasks: tasks(),
            rx,
            tx,
            ssh_env,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        self.start_initial_tasks();

        loop {
            terminal.draw(|frame| crate::ui::render(frame, &self.tasks))?;

            while let Ok(event) = self.rx.try_recv() {
                self.handle_runner_event(event);
            }

            if event::poll(Duration::from_millis(50))?
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
                && key.code == KeyCode::Char('q')
            {
                return Ok(());
            }

            if self.all_done() {
                tokio::time::sleep(Duration::from_millis(500)).await;
                break;
            }
        }

        terminal.draw(|frame| crate::ui::render(frame, &self.tasks))?;
        wait_for_quit()?;
        Ok(())
    }

    fn start_initial_tasks(&mut self) {
        let parallel_ids: Vec<_> = self
            .tasks
            .iter()
            .filter(|t| t.lane == Lane::Parallel)
            .map(|t| (t.id, t.command.clone()))
            .collect();

        for (id, command) in parallel_ids {
            self.start_task(id, command);
        }
    }

    fn start_task(&mut self, id: &'static str, command: crate::task::Command) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.state = State::Running;
        }
        let tx = self.tx.clone();
        let ssh_env = self.ssh_env.clone();
        tokio::spawn(async move {
            runner::run_task(id, command, tx, ssh_env).await;
        });
    }

    fn handle_runner_event(&mut self, event: runner::Event) {
        match event {
            runner::Event::Output(id, line) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.output.push(line);
                }
            }
            runner::Event::Completed(id, status) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.complete(status);
                }
                self.check_unblock(&id);
            }
            runner::Event::Failed(id, error) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.fail(error);
                }
                self.check_unblock(&id);
            }
        }
    }

    fn check_unblock(&mut self, completed_id: &str) {
        let next_task = self
            .tasks
            .iter()
            .find(|t| {
                matches!(t.state, State::Blocked)
                    && t.depends_on == Some(completed_id)
            })
            .map(|t| (t.id, t.command.clone()));

        if let Some((id, command)) = next_task {
            self.start_task(id, command);
        }
    }

    fn all_done(&self) -> bool {
        self.tasks.iter().all(|t| t.state.is_done())
    }
}

fn wait_for_quit() -> io::Result<()> {
    loop {
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && key.code == KeyCode::Char('q')
        {
            return Ok(());
        }
    }
}
