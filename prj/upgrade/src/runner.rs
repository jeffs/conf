use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc;

use crate::task::Command;

pub enum Event {
    Output(String, String),
    Completed(String, std::process::ExitStatus),
    Failed(String, String),
}

pub async fn run_task(
    id: &'static str,
    command: Command,
    tx: mpsc::UnboundedSender<Event>,
) {
    let result = match command {
        Command::Shell { program, args } => run_shell(id, program, args, tx.clone()).await,
        Command::CargoCrates => run_cargo_crates(id, tx.clone()).await,
        Command::Helix => run_helix(id, tx.clone()).await,
    };

    if let Err(e) = result {
        let _ = tx.send(Event::Failed(id.to_string(), e));
    }
}

/// Runs a command, streams output, and returns the exit status.
async fn run_cmd<I, S>(
    id: &str,
    program: &str,
    args: I,
    tx: mpsc::UnboundedSender<Event>,
) -> Result<std::process::ExitStatus, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut child = TokioCommand::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to spawn: {e}"))?;

    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let id_owned = id.to_string();
    let tx_stdout = tx.clone();
    let stdout_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = tx_stdout.send(Event::Output(id_owned.clone(), line));
        }
    });

    let id_owned = id.to_string();
    let tx_stderr = tx.clone();
    let stderr_handle = tokio::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let _ = tx_stderr.send(Event::Output(id_owned.clone(), line));
        }
    });

    let _ = stdout_handle.await;
    let _ = stderr_handle.await;

    child.wait().await.map_err(|e| format!("wait failed: {e}"))
}

/// Runs a command and sends the Completed event when done.
async fn run_shell<I, S>(
    id: &'static str,
    program: &str,
    args: I,
    tx: mpsc::UnboundedSender<Event>,
) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let status = run_cmd(id, program, args, tx.clone()).await?;
    let _ = tx.send(Event::Completed(id.to_string(), status));
    Ok(())
}

async fn run_cargo_crates(
    id: &'static str,
    tx: mpsc::UnboundedSender<Event>,
) -> Result<(), String> {
    run_shell(id, "cargo", &["install-update", "-a"], tx).await
}

async fn run_helix(
    id: &'static str,
    tx: mpsc::UnboundedSender<Event>,
) -> Result<(), String> {
    let home = std::env::var("HOME").map_err(|e| format!("HOME not set: {e}"))?;
    let helix_dir = format!("{home}/pkg/helix");

    if std::path::Path::new(&helix_dir).exists() {
        let _ = tx.send(Event::Output(id.to_string(), "Updating Helix fork...".to_string()));
        let status = run_cmd(
            id,
            "git",
            ["-C", &helix_dir, "pull", "--ff-only"],
            tx.clone(),
        )
        .await?;
        if !status.success() {
            let _ = tx.send(Event::Completed(id.to_string(), status));
            return Ok(());
        }
    } else {
        let _ = tx.send(Event::Output(id.to_string(), "Cloning Helix fork...".to_string()));
        let parent = format!("{home}/pkg");
        std::fs::create_dir_all(&parent).map_err(|e| format!("mkdir failed: {e}"))?;
        let status = run_cmd(
            id,
            "git",
            ["clone", "git@github.com:jeffs/helix.git", &helix_dir],
            tx.clone(),
        )
        .await?;
        if !status.success() {
            let _ = tx.send(Event::Completed(id.to_string(), status));
            return Ok(());
        }
    }

    let helix_term = format!("{helix_dir}/helix-term");
    run_shell(id, "cargo", ["install", "--path", &helix_term], tx).await
}
