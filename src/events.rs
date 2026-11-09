use crate::{Call, Failure};

/// Render parsed server-sent events, optionally selecting one exact event type.
pub fn run(call: &Call, input: &str) -> Result<String, Failure> {
    let selected_type = call.option("--type");
    let mut output = String::new();

    for (index, event) in crate::sse::parse(input).iter().enumerate() {
        if selected_type.is_some_and(|kind| event.kind != kind) {
            continue;
        }

        let id = event.id.as_deref().filter(|id| !id.is_empty()).unwrap_or("-");
        output.push_str(&(index + 1).to_string());
        output.push(' ');
        output.push_str(&event.kind);
        output.push_str(" id=");
        output.push_str(id);
        output.push_str(" data=");
        output.push_str(&escape_data(&event.data));
        output.push('\n');
    }

    Ok(output)
}

fn escape_data(data: &str) -> String {
    let mut escaped = String::with_capacity(data.len());
    for character in data.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            other => escaped.push(other),
        }
    }
    escaped
}
