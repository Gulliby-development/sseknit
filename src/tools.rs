use std::collections::BTreeMap;

use crate::{Call, Failure, Value};

#[derive(Default)]
struct ToolCall {
    name: Option<String>,
    args: String,
}

/// Assemble and render tool calls from the message events in an SSE stream.
pub fn run(_call: &Call, input: &str) -> Result<String, Failure> {
    let mut calls = BTreeMap::<i64, ToolCall>::new();

    for (event_index, event) in crate::sse::parse(input).iter().enumerate() {
        let number = event_index + 1;
        if event.kind == "error" {
            return Err(Failure::new(1, format!("event {number}: stream error: {}", event.data)));
        }
        if event.kind != "message" {
            continue;
        }
        if event.data == "[DONE]" {
            break;
        }

        let members = crate::json::parse_flat(&event.data).ok_or_else(|| Failure::new(1, format!("event {number}: invalid JSON")))?;
        let Some(index_value) = crate::json::get(&members, "index") else {
            continue;
        };
        let index = match index_value {
            Value::Int(index) if *index >= 0 => *index,
            _ => return Err(Failure::new(1, format!("event {number}: field index is not a non-negative integer"))),
        };
        let name = match crate::json::get(&members, "name") {
            Some(Value::Str(name)) => Some(name.as_str()),
            Some(_) => return Err(Failure::new(1, format!("event {number}: field name is not a string"))),
            None => None,
        };
        let args = match crate::json::get(&members, "args") {
            Some(Value::Str(args)) => Some(args.as_str()),
            Some(_) => return Err(Failure::new(1, format!("event {number}: field args is not a string"))),
            None => None,
        };

        let call = calls.entry(index).or_default();
        if let Some(name) = name.filter(|name| !name.is_empty()) {
            if let Some(old_name) = &call.name {
                if old_name != name {
                    return Err(Failure::new(1, format!("event {number}: tool {index} renamed from {old_name} to {name}")));
                }
            } else {
                call.name = Some(name.to_string());
            }
        }
        if let Some(args) = args {
            call.args.push_str(args);
        }
    }

    let mut output = String::new();
    for (index, call) in calls {
        output.push_str(&format!("call {index} {} {}", call.name.as_deref().unwrap_or("-"), if arguments_complete(&call.args) { "complete" } else { "incomplete" }));
        if !call.args.is_empty() {
            output.push(' ');
            output.push_str(&escape_args(&call.args));
        }
        output.push('\n');
    }
    Ok(output)
}

fn arguments_complete(args: &str) -> bool {
    let trimmed = args.trim_matches(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r'));
    if !matches!(trimmed.chars().next(), Some('{' | '[')) {
        return false;
    }

    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escaped = false;
    for ch in trimmed.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => stack.push(ch),
            '}' if stack.pop() != Some('{') => return false,
            ']' if stack.pop() != Some('[') => return false,
            '}' | ']' => {}
            _ => {}
        }
    }
    !in_string && !escaped && stack.is_empty()
}

fn escape_args(args: &str) -> String {
    let mut escaped = String::with_capacity(args.len());
    for ch in args.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped
}
