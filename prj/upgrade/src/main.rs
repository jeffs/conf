mod app;
mod runner;
mod task;
mod ui;

use std::io;
use std::process::{Command, Stdio};

use app::App;

#[derive(Clone, Default)]
pub struct SshEnv {
    pub auth_sock: Option<String>,
    pub agent_pid: Option<String>,
}

fn ensure_ssh_key() -> io::Result<SshEnv> {
    let status = Command::new("ssh-add")
        .arg("-l")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if status.success() {
        return Ok(SshEnv::default());
    }

    eprintln!("No SSH key loaded. Starting ssh-agent and prompting for passphrase...");
    let output = Command::new("ssh-agent").arg("-s").output()?;
    let agent_output = String::from_utf8_lossy(&output.stdout);

    let mut ssh_env = SshEnv::default();
    for line in agent_output.lines() {
        if let Some(rest) = line.strip_prefix("SSH_AUTH_SOCK=")
            && let Some(val) = rest.strip_suffix("; export SSH_AUTH_SOCK;")
        {
            ssh_env.auth_sock = Some(val.to_string());
        } else if let Some(rest) = line.strip_prefix("SSH_AGENT_PID=")
            && let Some(val) = rest.strip_suffix("; export SSH_AGENT_PID;")
        {
            ssh_env.agent_pid = Some(val.to_string());
        }
    }

    let mut cmd = Command::new("ssh-add");
    if let Some(ref sock) = ssh_env.auth_sock {
        cmd.env("SSH_AUTH_SOCK", sock);
    }
    if let Some(ref pid) = ssh_env.agent_pid {
        cmd.env("SSH_AGENT_PID", pid);
    }
    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::other("ssh-add failed"));
    }

    Ok(ssh_env)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let ssh_env = ensure_ssh_key()?;

    let terminal = ratatui::init();
    let result = App::new(ssh_env).run(terminal).await;
    ratatui::restore();
    result
}
