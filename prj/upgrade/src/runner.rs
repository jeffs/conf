use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc;

use crate::task::Command;
use crate::SshEnv;

pub enum Event {
    Output(String, String),
    Completed(String, std::process::ExitStatus),
    Failed(String, String),
}

pub async fn run_task(
    id: &'static str,
    command: Command,
    tx: mpsc::UnboundedSender<Event>,
    ssh_env: SshEnv,
) {
    let result = match command {
        Command::Shell { program, args } => run_shell(id, program, args, tx.clone(), None).await,
        Command::Script(path) => run_script(id, path, tx.clone(), &ssh_env).await,
        Command::CargoCrates => run_cargo_crates(id, tx.clone()).await,
    };

    if let Err(e) = result {
        let _ = tx.send(Event::Failed(id.to_string(), e));
    }
}

async fn run_shell(
    id: &'static str,
    program: &str,
    args: &[&str],
    tx: mpsc::UnboundedSender<Event>,
    ssh_env: Option<&SshEnv>,
) -> Result<(), String> {
    let mut cmd = TokioCommand::new(program);
    cmd.args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(env) = ssh_env {
        if let Some(ref sock) = env.auth_sock {
            cmd.env("SSH_AUTH_SOCK", sock);
        }
        if let Some(ref pid) = env.agent_pid {
            cmd.env("SSH_AGENT_PID", pid);
        }
    }

    let mut child = cmd.spawn().map_err(|e| format!("failed to spawn: {e}"))?;

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

    let status = child.wait().await.map_err(|e| format!("wait failed: {e}"))?;
    let _ = tx.send(Event::Completed(id.to_string(), status));
    Ok(())
}

async fn run_script(
    id: &'static str,
    path: &str,
    tx: mpsc::UnboundedSender<Event>,
    ssh_env: &SshEnv,
) -> Result<(), String> {
    let expanded = shellexpand::tilde(path);
    run_shell(id, &expanded, &[], tx, Some(ssh_env)).await
}

async fn run_cargo_crates(
    id: &'static str,
    tx: mpsc::UnboundedSender<Event>,
) -> Result<(), String> {
    let _ = tx.send(Event::Output(id.to_string(), "Fetching installed crates...".to_string()));

    let output = TokioCommand::new("cargo")
        .args(["install", "--list"])
        .output()
        .await
        .map_err(|e| format!("failed to list crates: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let crates: Vec<&str> = stdout
        .lines()
        .filter(|line| !line.starts_with(' ') && !line.contains('('))
        .filter_map(|line| line.split_whitespace().next())
        .collect();

    if crates.is_empty() {
        let _ = tx.send(Event::Output(id.to_string(), "No crates to update".to_string()));
        let status = std::process::ExitStatus::default();
        let _ = tx.send(Event::Completed(id.to_string(), status));
        return Ok(());
    }

    let _ = tx.send(Event::Output(
        id.to_string(),
        format!("Installing {} crates: {}", crates.len(), crates.join(", ")),
    ));

    run_shell(id, "cargo", &[&["install"], crates.as_slice()].concat(), tx, None).await
}
