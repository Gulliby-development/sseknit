use crate::registry::{self, COMMANDS};
use crate::{Call, Failure, Outcome};

/// Parses, validates and dispatches one command line.
pub fn run(args: &[String], input: &str) -> Outcome {
    if args.is_empty() {
        return Outcome { code: 2, out: String::new(), err: registry::USAGE.to_string() };
    }

    match args[0].as_str() {
        "help" | "-h" | "--help" => {
            return Outcome { code: 0, out: registry::USAGE.to_string(), err: String::new() };
        }
        "--version" => {
            return Outcome { code: 0, out: "sseknit 0.1.0\n".to_string(), err: String::new() };
        }
        _ => {}
    }

    let Some(command) = COMMANDS.iter().find(|command| command.name == args[0]) else {
        return failure(Failure::new(2, format!("unknown command {}", args[0])));
    };

    let mut call = Call::default();
    let mut index = 1;
    while index < args.len() {
        let word = &args[index];
        if word.starts_with('-') && word.len() > 1 {
            if !command.values.contains(&word.as_str()) {
                return failure(Failure::new(2, format!("unknown option {word}")));
            }
            if call.option(word).is_some() {
                return failure(Failure::new(2, format!("{word} given twice")));
            }
            index += 1;
            if index == args.len() || args[index].is_empty() {
                return failure(Failure::new(2, format!("{word} needs a non-empty name")));
            }
            call.options.push((word.clone(), args[index].clone()));
        } else {
            if call.positional.len() >= command.max_positional {
                return failure(Failure::new(2, format!("unexpected argument {word}")));
            }
            call.positional.push(word.clone());
        }
        index += 1;
    }

    if call.positional.len() < command.min_positional {
        return failure(Failure::new(2, format!("{} needs {}", command.name, command.names)));
    }
    if command.name == "stop" && call.positional.iter().any(String::is_empty) {
        return failure(Failure::new(2, "stop needs non-empty stop sequences"));
    }
    if command.name == "rechunk" && !valid_rechunk_size(&call.positional[0]) {
        return failure(Failure::new(2, "rechunk needs an integer from 1 to 200"));
    }

    match (command.handler)(&call, input) {
        Ok(out) => Outcome { code: 0, out, err: String::new() },
        Err(error) => failure(error),
    }
}

fn failure(error: Failure) -> Outcome {
    Outcome { code: error.code, out: String::new(), err: format!("sseknit: {}\n", error.message) }
}

fn valid_rechunk_size(value: &str) -> bool {
    if value.is_empty() || value.len() > 5 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    value.parse::<u16>().is_ok_and(|size| (1..=200).contains(&size))
}
