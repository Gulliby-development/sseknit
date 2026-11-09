//! sseknit: assemble streamed model output from server-sent events.
pub mod cli;
pub mod events;
pub mod json;
pub mod metrics;
pub mod rechunk;
pub mod registry;
pub mod sse;
pub mod text;
pub mod tools;

/// An error with its exit status (2 for the command line, 1 for the stream) and its message.
#[derive(Debug)]
pub struct Failure {
    pub code: i32,
    pub message: String,
}

impl Failure {
    pub fn new(code: i32, message: impl Into<String>) -> Failure {
        Failure { code, message: message.into() }
    }
}

/// What a run gives back: the exit status and the two output strings.
#[derive(Debug, PartialEq)]
pub struct Outcome {
    pub code: i32,
    pub out: String,
    pub err: String,
}

/// One dispatched server-sent event: its type (`message` unless an `event` field set it), the last event id seen so far, and its data lines joined by LF.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub kind: String,
    pub id: Option<String>,
    pub data: String,
}

/// A value of the flat JSON objects in the data of an event (`Num` is a number that is not an integer).
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Num,
    Bool(bool),
    Null,
}

/// The parsed command line of one command: the options given (name and value) and the positional words.
#[derive(Debug, Default)]
pub struct Call {
    pub options: Vec<(String, String)>,
    pub positional: Vec<String>,
}

impl Call {
    pub fn option(&self, name: &str) -> Option<&str> {
        self.options.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
    }
}

pub type Handler = fn(&Call, &str) -> Result<String, Failure>;

/// A row of the command table: the options that take a value, and how many positional words the command takes.
pub struct Command {
    pub name: &'static str,
    pub handler: Handler,
    pub values: &'static [&'static str],
    pub min_positional: usize,
    pub max_positional: usize,
    pub names: &'static str,
}

pub fn run(args: &[String], input: &str) -> Outcome {
    cli::run(args, input)
}

/// True when the program has to read standard input for this command line.
pub fn reads_stdin(args: &[String]) -> bool {
    !matches!(args.first().map(String::as_str), None | Some("help") | Some("-h") | Some("--help") | Some("--version"))
}
