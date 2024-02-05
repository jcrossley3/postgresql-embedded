use crate::error::Result;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

/// Trait to build a command
pub trait CommandBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr;

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf>;

    /// Fully qualified path to the program binary
    fn get_program_file(&self) -> PathBuf {
        let program_name = &self.get_program();
        match self.get_program_dir() {
            Some(program_dir) => program_dir.join(program_name),
            None => PathBuf::from(program_name),
        }
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString>;

    /// Build a standard Command
    fn build(self) -> std::process::Command
    where
        Self: Sized,
    {
        let program_file = self.get_program_file();
        let mut command = std::process::Command::new(program_file);

        command.args(self.get_args());
        command
    }

    #[cfg(feature = "tokio")]
    /// Build a tokio Command
    fn build_tokio(self) -> tokio::process::Command
    where
        Self: Sized,
    {
        let program_file = self.get_program_file();
        let mut command = tokio::process::Command::new(program_file);

        command.args(self.get_args());
        command
    }
}

/// Trait to convert a command to a string representation
pub trait CommandToString {
    fn to_command_string(&self) -> String;
}

/// Implement the [`CommandToString`] trait for [`Command`](std::process::Command)
impl CommandToString for std::process::Command {
    fn to_command_string(&self) -> String {
        format!("{self:?}")
    }
}

#[cfg(feature = "tokio")]
/// Implement the [`CommandToString`] trait for [`Command`](tokio::process::Command)
impl CommandToString for tokio::process::Command {
    fn to_command_string(&self) -> String {
        format!("{self:?}")
            .replace("Command { std: ", "")
            .replace(", kill_on_drop: false }", "")
    }
}

pub trait CommandExecutor {
    async fn execute(&mut self, timeout: Option<Duration>) -> Result<(String, String)>;
}

/// Implement the [`CommandExecutor`] trait for [`Command`](std::process::Command)
impl CommandExecutor for std::process::Command {
    async fn execute(&mut self, _timeout: Option<Duration>) -> Result<(String, String)> {
        self.stdout(Stdio::piped());
        self.stderr(Stdio::piped());

        let output = self.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        Ok((stdout, stderr))
    }
}

#[cfg(feature = "tokio")]
/// Implement the [`CommandExecutor`] trait for [`Command`](tokio::process::Command)
impl CommandExecutor for tokio::process::Command {
    async fn execute(&mut self, timeout: Option<Duration>) -> Result<(String, String)> {
        self.stdout(Stdio::piped());
        self.stderr(Stdio::piped());

        let output = match timeout {
            Some(duration) => tokio::time::timeout(duration, self.output()).await?,
            None => self.output().await,
        }?;

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        if output.status.success() {
            Ok((stdout, stderr))
        } else {
            Err(crate::EmbeddedError::CommandError { stdout, stderr })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestCommandBuilder {
        program: OsString,
        program_dir: Option<PathBuf>,
        args: Vec<OsString>,
    }

    impl CommandBuilder for TestCommandBuilder {
        fn get_program(&self) -> &'static OsStr {
            "test".as_ref()
        }

        fn get_program_dir(&self) -> &Option<PathBuf> {
            &self.program_dir
        }

        fn get_args(&self) -> Vec<OsString> {
            self.args.clone()
        }
    }

    #[test]
    fn test_standard_command_builder() {
        let builder = TestCommandBuilder {
            program: "ls".to_string().into(),
            program_dir: Some("/tmp".to_string().into()),
            args: vec!["--help".to_string().into()],
        };
        let command = builder.build();

        assert_eq!(r#""/tmp/test" "--help""#, command.to_command_string());
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn test_tokio_command_builder() {
        let builder = TestCommandBuilder {
            program: "ls".to_string().into(),
            program_dir: Some("/tmp".to_string().into()),
            args: vec!["--help".to_string().into()],
        };
        let command = builder.build_tokio();

        assert_eq!(r#""/tmp/test" "--help""#, command.to_command_string());
    }

    #[test]
    fn test_standard_to_command_string() {
        let mut command = std::process::Command::new("foo".to_string());
        command.current_dir("/tmp".to_string());
        command.arg("-l".to_string());
        assert_eq!(r#"cd "/tmp" && "foo" "-l""#, command.to_command_string(),);
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn test_tokio_to_command_string() {
        let mut command = tokio::process::Command::new("foo".to_string());
        command.current_dir("/tmp".to_string());
        command.arg("-l".to_string());
        assert_eq!(r#"cd "/tmp" && "foo" "-l""#, command.to_command_string(),);
    }

    #[tokio::test]
    async fn test_standard_command_execute() -> Result<()> {
        #[cfg(not(target_os = "windows"))]
        let mut command = tokio::process::Command::new("ls");

        #[cfg(target_os = "windows")]
        let mut command = tokio::process::Command::new("dir");

        let (stdout, stderr) = command.execute(None).await?;
        assert!(!stdout.is_empty());
        assert!(stderr.is_empty());
        Ok(())
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn test_tokio_command_execute() -> Result<()> {
        #[cfg(not(target_os = "windows"))]
        let mut command = tokio::process::Command::new("ls");

        #[cfg(target_os = "windows")]
        let mut command = tokio::process::Command::new("dir");

        let (stdout, stderr) = command.execute(None).await?;
        assert!(!stdout.is_empty());
        assert!(stderr.is_empty());
        Ok(())
    }
}