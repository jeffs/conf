use std::process::ExitStatus;

#[derive(Clone, Debug)]
pub enum State {
    Pending,
    Blocked,
    Running,
    Completed,
    Failed(String),
}

impl State {
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::Pending | Self::Blocked => "○",
            Self::Running => "◐",
            Self::Completed => "✓",
            Self::Failed(_) => "✗",
        }
    }

    pub const fn is_done(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed(_))
    }
}

#[derive(Clone, Debug)]
pub enum Command {
    Shell { program: &'static str, args: &'static [&'static str] },
    Script(&'static str),
    CargoCrates,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: &'static str,
    pub label: &'static str,
    pub command: Command,
    pub state: State,
    pub output: Vec<String>,
    pub depends_on: Option<&'static str>,
    pub lane: Lane,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lane {
    Parallel,
    Cargo,
}

impl Task {
    pub fn complete(&mut self, status: ExitStatus) {
        self.state = if status.success() {
            State::Completed
        } else {
            State::Failed(format!("exit code {}", status.code().unwrap_or(-1)))
        };
    }

    pub fn fail(&mut self, error: String) {
        self.state = State::Failed(error);
    }
}

pub fn tasks() -> Vec<Task> {
    vec![
        Task {
            id: "brew",
            label: "brew upgrade",
            command: Command::Shell {
                program: "brew",
                args: &["upgrade", "--quiet"],
            },
            state: State::Pending,
            output: Vec::new(),
            depends_on: None,
            lane: Lane::Parallel,
        },
        Task {
            id: "rustup",
            label: "rustup update",
            command: Command::Shell {
                program: "rustup",
                args: &["update"],
            },
            state: State::Pending,
            output: Vec::new(),
            depends_on: None,
            lane: Lane::Parallel,
        },
        Task {
            id: "uv",
            label: "uv tool install",
            command: Command::Shell {
                program: "uv",
                args: &[
                    "tool",
                    "install",
                    "specify-cli",
                    "--force",
                    "--from",
                    "git+https://github.com/github/spec-kit.git",
                ],
            },
            state: State::Pending,
            output: Vec::new(),
            depends_on: None,
            lane: Lane::Parallel,
        },
        Task {
            id: "softwareupdate",
            label: "softwareupdate",
            command: Command::Shell {
                program: "softwareupdate",
                args: &["--list"],
            },
            state: State::Pending,
            output: Vec::new(),
            depends_on: None,
            lane: Lane::Parallel,
        },
        Task {
            id: "cargo",
            label: "cargo install",
            command: Command::CargoCrates,
            state: State::Blocked,
            output: Vec::new(),
            depends_on: Some("rustup"),
            lane: Lane::Cargo,
        },
        Task {
            id: "nushell",
            label: "install-nushell.zsh",
            command: Command::Script("~/conf/src/install-nushell.zsh"),
            state: State::Blocked,
            output: Vec::new(),
            depends_on: Some("cargo"),
            lane: Lane::Cargo,
        },
        Task {
            id: "helix",
            label: "install-helix.nu",
            command: Command::Script("~/conf/src/install-helix.nu"),
            state: State::Blocked,
            output: Vec::new(),
            depends_on: Some("nushell"),
            lane: Lane::Cargo,
        },
    ]
}
