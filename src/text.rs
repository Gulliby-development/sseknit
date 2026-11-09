use crate::{Call, Failure, Value};

struct Piece {
    start: usize,
    end: usize,
    event: usize,
}

struct Collected {
    content: String,
    pieces: Vec<Piece>,
}

fn collect(call: &Call, input: &str) -> Result<Collected, Failure> {
    let field = call.option("--field").unwrap_or("delta");
    let selected_type = call.option("--event").unwrap_or("message");
    let mut result = Collected { content: String::new(), pieces: Vec::new() };

    for (index, event) in crate::sse::parse(input).iter().enumerate() {
        let number = index + 1;
        if event.kind == "error" {
            return Err(Failure::new(1, format!("event {number}: stream error: {}", event.data)));
        }
        if event.kind != selected_type {
            continue;
        }
        if event.data == "[DONE]" {
            break;
        }
        let members = crate::json::parse_flat(&event.data).ok_or_else(|| Failure::new(1, format!("event {number}: invalid JSON")))?;
        if let Some(value) = crate::json::get(&members, field) {
            let Value::Str(text) = value else {
                return Err(Failure::new(1, format!("event {number}: field {field} is not a string")));
            };
            if !text.is_empty() {
                let start = result.content.len();
                result.content.push_str(text);
                result.pieces.push(Piece { start, end: result.content.len(), event: number });
            }
        }
    }
    Ok(result)
}

/// Assemble the selected text fields and append one line feed.
pub fn text(call: &Call, input: &str) -> Result<String, Failure> {
    let mut content = collect(call, input)?.content;
    content.push('\n');
    Ok(content)
}

/// Assemble selected text and cut it at the earliest, longest stop match.
pub fn stop(call: &Call, input: &str) -> Result<String, Failure> {
    let collected = collect(call, input)?;
    let mut best: Option<(&str, usize, usize)> = None;

    for sequence in &call.positional {
        if sequence.is_empty() {
            continue;
        }
        let mut from = 0;
        while let Some(relative) = collected.content[from..].find(sequence) {
            let start = from + relative;
            let length = sequence.chars().count();
            if best.is_none_or(|(_, best_start, best_length)| start < best_start || (start == best_start && length > best_length)) {
                best = Some((sequence, start, length));
            }
            from = start + collected.content[start..].chars().next().map(char::len_utf8).unwrap_or(1);
        }
    }

    let mut output = String::new();
    if let Some((sequence, start, _)) = best {
        output.push_str(&collected.content[..start]);
        output.push('\n');
        let end = start + sequence.len();
        let event = collected.pieces.iter().find(|piece| end <= piece.end && end > piece.start).map(|piece| piece.event).unwrap_or(0);
        let offset = collected.content[..start].chars().count();
        output.push_str(&format!("-- stopped by [{sequence}] at offset {offset} in event {event}\n"));
    } else {
        output.push_str(&collected.content);
        output.push_str("\n-- no stop sequence matched\n");
    }
    Ok(output)
}
