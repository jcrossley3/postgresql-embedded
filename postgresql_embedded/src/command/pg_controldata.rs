use crate::command::traits::CommandBuilder;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// pg_controldata displays control information of a PostgreSQL database cluster.
#[derive(Clone, Debug, Default)]
pub struct PgControlDataBuilder {
    program_dir: Option<PathBuf>,
    pgdata: Option<PathBuf>,
    version: bool,
    help: bool,
}

impl PgControlDataBuilder {
    /// Create a new [`PgControlDataBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// Set the data directory
    pub fn pgdata<P: Into<PathBuf>>(mut self, pgdata: P) -> Self {
        self.pgdata = Some(pgdata.into());
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// show help, then exit
    pub fn help(mut self) -> Self {
        self.help = true;
        self
    }
}

impl CommandBuilder for PgControlDataBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "pg_controldata".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(pgdata) = &self.pgdata {
            args.push("--pgdata".into());
            args.push(pgdata.into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.help {
            args.push("--help".into());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;

    #[test]
    fn test_builder_new() {
        let command = PgControlDataBuilder::new().build();

        assert_eq!(r#""pg_controldata""#, command.to_command_string());
    }

    #[test]
    fn test_builder() {
        let command = PgControlDataBuilder::new()
            .program_dir("/usr/bin")
            .pgdata("/var/lib/postgresql/data")
            .version()
            .help()
            .build();

        assert_eq!(
            r#""/usr/bin/pg_controldata" "--pgdata" "/var/lib/postgresql/data" "--version" "--help""#,
            command.to_command_string()
        );
    }
}