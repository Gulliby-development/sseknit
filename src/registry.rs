use crate::events;
use crate::{metrics, rechunk, text, tools, Command};

pub const USAGE: &str = "usage: sseknit events [--type T]\n       sseknit text [--field F] [--event T]\n       sseknit stop [--field F] [--event T] STOP...\n       sseknit usage\n       sseknit tools\n       sseknit timeline\n       sseknit rechunk N\n";

pub const COMMANDS: [Command; 7] = [
    Command { name: "events", handler: events::run, values: &["--type"], min_positional: 0, max_positional: 0, names: "" },
    Command { name: "text", handler: text::text, values: &["--field", "--event"], min_positional: 0, max_positional: 0, names: "" },
    Command { name: "stop", handler: text::stop, values: &["--field", "--event"], min_positional: 1, max_positional: usize::MAX, names: "STOP..." },
    Command { name: "usage", handler: metrics::usage, values: &[], min_positional: 0, max_positional: 0, names: "" },
    Command { name: "tools", handler: tools::run, values: &[], min_positional: 0, max_positional: 0, names: "" },
    Command { name: "timeline", handler: metrics::timeline, values: &[], min_positional: 0, max_positional: 0, names: "" },
    Command { name: "rechunk", handler: rechunk::run, values: &[], min_positional: 1, max_positional: 1, names: "N" },
];
