use crate::command::traits::CommandBuilder;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// psql is the PostgreSQL interactive terminal.
#[derive(Clone, Debug, Default)]
pub struct PsqlBuilder {
    program_dir: Option<PathBuf>,
    command: Option<OsString>,
    dbname: Option<OsString>,
    file: Option<PathBuf>,
    list: bool,
    variable: Option<(OsString, OsString)>,
    version: bool,
    no_psqlrc: bool,
    single_transaction: bool,
    help: Option<OsString>,
    echo_all: bool,
    echo_errors: bool,
    echo_queries: bool,
    echo_hidden: bool,
    log_file: Option<PathBuf>,
    no_readline: bool,
    output: Option<PathBuf>,
    quiet: bool,
    single_step: bool,
    single_line: bool,
    no_align: bool,
    csv: bool,
    field_separator: Option<OsString>,
    html: bool,
    pset: Option<(OsString, OsString)>,
    record_separator: Option<OsString>,
    tuples_only: bool,
    table_attr: Option<OsString>,
    expanded: bool,
    field_separator_zero: bool,
    record_separator_zero: bool,
    host: Option<OsString>,
    port: Option<u16>,
    username: Option<OsString>,
    no_password: bool,
    password: bool,
}

impl PsqlBuilder {
    /// Create a new [`PsqlBuilder`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Location of the program binary
    pub fn program_dir<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.program_dir = Some(path.into());
        self
    }

    /// run only single command (SQL or internal) and exit
    pub fn command<S: AsRef<OsStr>>(mut self, command: S) -> Self {
        self.command = Some(command.as_ref().to_os_string());
        self
    }

    /// database name to connect to
    pub fn dbname<S: AsRef<OsStr>>(mut self, dbname: S) -> Self {
        self.dbname = Some(dbname.as_ref().to_os_string());
        self
    }

    /// execute commands from file, then exit
    pub fn file<P: Into<PathBuf>>(mut self, file: P) -> Self {
        self.file = Some(file.into());
        self
    }

    /// list available databases, then exit
    pub fn list(mut self) -> Self {
        self.list = true;
        self
    }

    /// set psql variable NAME to VALUE (e.g., -v ON_ERROR_STOP=1)
    pub fn variable<S: AsRef<OsStr>>(mut self, variable: (S, S)) -> Self {
        let (name, value) = variable;
        self.variable = Some((name.as_ref().into(), value.as_ref().into()));
        self
    }

    /// output version information, then exit
    pub fn version(mut self) -> Self {
        self.version = true;
        self
    }

    /// do not read startup file (~/.psqlrc)
    pub fn no_psqlrc(mut self) -> Self {
        self.no_psqlrc = true;
        self
    }

    /// execute as a single transaction (if non-interactive)
    pub fn single_transaction(mut self) -> Self {
        self.single_transaction = true;
        self
    }

    /// show help, then exit
    /// Possible values: [options, commands, variables]
    pub fn help<S: AsRef<OsStr>>(mut self, help: S) -> Self {
        self.help = Some(help.as_ref().to_os_string());
        self
    }

    /// echo all input from script
    pub fn echo_all(mut self) -> Self {
        self.echo_all = true;
        self
    }

    /// echo failed commands
    pub fn echo_errors(mut self) -> Self {
        self.echo_errors = true;
        self
    }

    /// echo commands sent to server
    pub fn echo_queries(mut self) -> Self {
        self.echo_queries = true;
        self
    }

    /// display queries that internal commands generate
    pub fn echo_hidden(mut self) -> Self {
        self.echo_hidden = true;
        self
    }

    /// send session log to file
    pub fn log_file<P: Into<PathBuf>>(mut self, log_file: P) -> Self {
        self.log_file = Some(log_file.into());
        self
    }

    /// disable enhanced command line editing (readline)
    pub fn no_readline(mut self) -> Self {
        self.no_readline = true;
        self
    }

    /// send query results to file (or |pipe)
    pub fn output<P: Into<PathBuf>>(mut self, output: P) -> Self {
        self.output = Some(output.into());
        self
    }

    /// run quietly (no messages, only query output)
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }

    /// single-step mode (confirm each query)
    pub fn single_step(mut self) -> Self {
        self.single_step = true;
        self
    }

    /// single-line mode (end of line terminates SQL command)
    pub fn single_line(mut self) -> Self {
        self.single_line = true;
        self
    }

    /// unaligned table output mode
    pub fn no_align(mut self) -> Self {
        self.no_align = true;
        self
    }

    /// CSV (Comma-Separated Values) table output mode
    pub fn csv(mut self) -> Self {
        self.csv = true;
        self
    }

    /// field separator for unaligned output (default: "|")
    pub fn field_separator<S: AsRef<OsStr>>(mut self, field_separator: S) -> Self {
        self.field_separator = Some(field_separator.as_ref().to_os_string());
        self
    }

    /// HTML table output mode
    pub fn html(mut self) -> Self {
        self.html = true;
        self
    }

    /// set printing option VAR to ARG (see \pset command)
    pub fn pset<S: AsRef<OsStr>>(mut self, pset: (S, S)) -> Self {
        let (var, arg) = pset;
        self.pset = Some((var.as_ref().into(), arg.as_ref().into()));
        self
    }

    /// record separator for unaligned output (default: newline)
    pub fn record_separator<S: AsRef<OsStr>>(mut self, record_separator: S) -> Self {
        self.record_separator = Some(record_separator.as_ref().to_os_string());
        self
    }

    /// print rows only
    pub fn tuples_only(mut self) -> Self {
        self.tuples_only = true;
        self
    }

    /// set HTML table tag attributes (e.g., width, border)
    pub fn table_attr<S: AsRef<OsStr>>(mut self, table_attr: S) -> Self {
        self.table_attr = Some(table_attr.as_ref().to_os_string());
        self
    }

    /// turn on expanded table output
    pub fn expanded(mut self) -> Self {
        self.expanded = true;
        self
    }

    /// set field separator for unaligned output to zero byte
    pub fn field_separator_zero(mut self) -> Self {
        self.field_separator_zero = true;
        self
    }

    /// set record separator for unaligned output to zero byte
    pub fn record_separator_zero(mut self) -> Self {
        self.record_separator_zero = true;
        self
    }

    /// database server host or socket directory
    pub fn host<S: AsRef<OsStr>>(mut self, host: S) -> Self {
        self.host = Some(host.as_ref().to_os_string());
        self
    }

    /// database server port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// database user name
    pub fn username<S: AsRef<OsStr>>(mut self, username: S) -> Self {
        self.username = Some(username.as_ref().to_os_string());
        self
    }

    /// never prompt for password
    pub fn no_password(mut self) -> Self {
        self.no_password = true;
        self
    }

    /// force password prompt (should happen automatically)
    pub fn password(mut self) -> Self {
        self.password = true;
        self
    }
}

impl CommandBuilder for PsqlBuilder {
    /// Get the program name
    fn get_program(&self) -> &'static OsStr {
        "psql".as_ref()
    }

    /// Location of the program binary
    fn get_program_dir(&self) -> &Option<PathBuf> {
        &self.program_dir
    }

    /// Get the arguments for the command
    fn get_args(&self) -> Vec<OsString> {
        let mut args: Vec<OsString> = Vec::new();

        if let Some(psql_command) = &self.command {
            args.push("--command".into());
            args.push(psql_command.into());
        }

        if let Some(dbname) = &self.dbname {
            args.push("--dbname".into());
            args.push(dbname.into());
        }

        if let Some(file) = &self.file {
            args.push("--file".into());
            args.push(file.into());
        }

        if self.list {
            args.push("--list".into());
        }

        if let Some((name, value)) = &self.variable {
            args.push("--variable".into());
            args.push(format!("{}={}", name.to_string_lossy(), value.to_string_lossy()).into());
        }

        if self.version {
            args.push("--version".into());
        }

        if self.no_psqlrc {
            args.push("--no-psqlrc".into());
        }

        if self.single_transaction {
            args.push("--single-transaction".into());
        }

        if let Some(help) = &self.help {
            args.push("--help".into());
            args.push(help.into());
        }

        if self.echo_all {
            args.push("--echo-all".into());
        }

        if self.echo_errors {
            args.push("--echo-errors".into());
        }

        if self.echo_queries {
            args.push("--echo-queries".into());
        }

        if self.echo_hidden {
            args.push("--echo-hidden".into());
        }

        if let Some(log_file) = &self.log_file {
            args.push("--log-file".into());
            args.push(log_file.into());
        }

        if self.no_readline {
            args.push("--no-readline".into());
        }

        if let Some(output) = &self.output {
            args.push("--output".into());
            args.push(output.into());
        }

        if self.quiet {
            args.push("--quiet".into());
        }

        if self.single_step {
            args.push("--single-step".into());
        }

        if self.single_line {
            args.push("--single-line".into());
        }

        if self.no_align {
            args.push("--no-align".into());
        }

        if self.csv {
            args.push("--csv".into());
        }

        if let Some(field_separator) = &self.field_separator {
            args.push("--field-separator".into());
            args.push(field_separator.into());
        }

        if self.html {
            args.push("--html".into());
        }

        if let Some((var, arg)) = &self.pset {
            args.push("--pset".into());
            args.push(format!("{}={}", var.to_string_lossy(), arg.to_string_lossy()).into());
        }

        if let Some(record_separator) = &self.record_separator {
            args.push("--record-separator".into());
            args.push(record_separator.into());
        }

        if self.tuples_only {
            args.push("--tuples-only".into());
        }

        if let Some(table_attr) = &self.table_attr {
            args.push("--table-attr".into());
            args.push(table_attr.into());
        }

        if self.expanded {
            args.push("--expanded".into());
        }

        if self.field_separator_zero {
            args.push("--field-separator-zero".into());
        }

        if self.record_separator_zero {
            args.push("--record-separator-zero".into());
        }

        if let Some(host) = &self.host {
            args.push("--host".into());
            args.push(host.into());
        }

        if let Some(port) = &self.port {
            args.push("--port".into());
            args.push(port.to_string().into());
        }

        if let Some(username) = &self.username {
            args.push("--username".into());
            args.push(username.into());
        }

        if self.no_password {
            args.push("--no-password".into());
        }

        if self.password {
            args.push("--password".into());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::traits::CommandToString;
    use test_log::test;

    #[test]
    fn test_builder_new() {
        let command = PsqlBuilder::new().program_dir(".").build();

        assert_eq!(
            PathBuf::from(".").join("psql"),
            PathBuf::from(command.to_command_string().replace("\"", ""))
        );
    }

    #[test]
    fn test_builder() {
        let command = PsqlBuilder::new()
            .command("SELECT * FROM test")
            .dbname("dbname")
            .file("test.sql")
            .list()
            .variable(("ON_ERROR_STOP", "1"))
            .version()
            .no_psqlrc()
            .single_transaction()
            .help("options")
            .echo_all()
            .echo_errors()
            .echo_queries()
            .echo_hidden()
            .log_file("psql.log")
            .no_readline()
            .output("output.txt")
            .quiet()
            .single_step()
            .single_line()
            .no_align()
            .csv()
            .field_separator("|")
            .html()
            .pset(("border", "1"))
            .record_separator("\n")
            .tuples_only()
            .table_attr("width=100")
            .expanded()
            .field_separator_zero()
            .record_separator_zero()
            .host("localhost")
            .port(5432)
            .username("postgres")
            .no_password()
            .password()
            .build();

        assert_eq!(
            r#""psql" "--command" "SELECT * FROM test" "--dbname" "dbname" "--file" "test.sql" "--list" "--variable" "ON_ERROR_STOP=1" "--version" "--no-psqlrc" "--single-transaction" "--help" "options" "--echo-all" "--echo-errors" "--echo-queries" "--echo-hidden" "--log-file" "psql.log" "--no-readline" "--output" "output.txt" "--quiet" "--single-step" "--single-line" "--no-align" "--csv" "--field-separator" "|" "--html" "--pset" "border=1" "--record-separator" "\n" "--tuples-only" "--table-attr" "width=100" "--expanded" "--field-separator-zero" "--record-separator-zero" "--host" "localhost" "--port" "5432" "--username" "postgres" "--no-password" "--password""#,
            command.to_command_string()
        );
    }
}
