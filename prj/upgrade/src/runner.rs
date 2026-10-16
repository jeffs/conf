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

pub async fn run_task(id: &'static str, command: Command, tx: mpsc::UnboundedSender<Event>) {
    let result = match command {
        Command::Shell { program, args } => run_shell(id, program, args, tx.clone()).await,
        Command::CargoCrates => run_cargo_crates(id, tx.clone()).await,
        Command::Rustup => run_rustup(id, tx.clone()).await,
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
    // `cargo-nextest` in particular refuses to install without `--locked`; see:
    // <https://nexte.st/changelog/#0.9.124>
    run_shell(id, "cargo", &["install-update", "--all", "--locked"], tx).await
}

/// Updates release channels, skipping nightly, then rustup itself.
async fn run_rustup(id: &'static str, tx: mpsc::UnboundedSender<Event>) -> Result<(), String> {
    let toolchains = release_channels().await?;
    let mut args = vec!["update", "--no-self-update"];
    args.extend(toolchains.iter().map(String::as_str));
    let status = run_cmd(id, "rustup", &args, tx.clone()).await?;
    if !status.success() {
        let _ = tx.send(Event::Completed(id.to_string(), status));
        return Ok(());
    }
    run_shell(id, "rustup", &["self", "update"], tx).await
}

/// Installed toolchains that track a stable or beta release channel, such as
/// `stable-aarch64-apple-darwin` or `1.90-aarch64-apple-darwin`.  Excludes
/// nightly, full versions like `1.90.0`, which never change, and custom
/// toolchains.
async fn release_channels() -> Result<Vec<String>, String> {
    let output = TokioCommand::new("rustup")
        .args(["toolchain", "list", "--quiet"])
        .output()
        .await
        .map_err(|e| format!("failed to list toolchains: {e}"))?;
    if !output.status.success() {
        return Err(format!("toolchain list failed: {}", output.status));
    }
    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("toolchain list is not UTF-8: {e}"))?;
    Ok(stdout
        .lines()
        .filter(|name| tracks_release_channel(name))
        .map(String::from)
        .collect())
}

fn tracks_release_channel(toolchain: &str) -> bool {
    let Some((channel, _host)) = toolchain.split_once('-') else {
        return false;
    };
    match channel {
        "stable" | "beta" => true,
        _ => channel
            .split_once('.')
            .is_some_and(|(major, minor)| is_number(major) && is_number(minor)),
    }
}

fn is_number(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}
