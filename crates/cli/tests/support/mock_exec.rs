use std::cell::RefCell;
use std::io;
use std::process::{Command, Output};

use crate::build::CommandExecutor;

pub struct MockCommandExecutor {
    script: RefCell<Vec<(Vec<String>, Output)>>,
}

impl MockCommandExecutor {
    pub fn new(script: Vec<(Vec<String>, Output)>) -> Self {
        Self {
            script: RefCell::new(script),
        }
    }
}

impl CommandExecutor for MockCommandExecutor {
    fn run(&self, cmd: &mut Command) -> io::Result<Output> {
        let args = std::iter::once(cmd.get_program().to_string_lossy().into_owned())
            .chain(cmd.get_args().map(|a| a.to_string_lossy().into_owned()))
            .collect::<Vec<_>>();
        let mut script = self.script.borrow_mut();
        if script.is_empty() {
            return Err(io::Error::other("unexpected command"));
        }
        let (expect, output) = script.remove(0);
        assert_eq!(expect, args, "command arguments mismatch");
        Ok(output)
    }
}

#[cfg(unix)]
pub fn success() -> std::process::ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(0)
}

#[cfg(windows)]
pub fn success() -> std::process::ExitStatus {
    use std::os::windows::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(0)
}
