use crate::Event;

/// Parse a server-sent event stream into its closed data-bearing events.
pub fn parse(text: &str) -> Vec<Event> {
    let mut events = Vec::new();
    let mut kind: Option<String> = None;
    let mut id: Option<String> = None;
    let mut data = String::new();
    let mut has_data = false;

    let mut start = 0;
    let bytes = text.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\n' || bytes[index] == b'\r' {
            process_line(&text[start..index], &mut events, &mut kind, &mut id, &mut data, &mut has_data);
            if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
                index += 1;
            }
            index += 1;
            start = index;
        } else {
            index += 1;
        }
    }

    if start < text.len() {
        process_line(&text[start..], &mut events, &mut kind, &mut id, &mut data, &mut has_data);
    }

    events
}

fn process_line(line: &str, events: &mut Vec<Event>, kind: &mut Option<String>, id: &mut Option<String>, data: &mut String, has_data: &mut bool) {
    if line.is_empty() {
        if *has_data {
            data.pop();
            events.push(Event { kind: kind.take().unwrap_or_else(|| "message".to_string()), id: id.clone(), data: std::mem::take(data) });
        } else {
            *kind = None;
            data.clear();
        }
        *has_data = false;
        return;
    }

    if line.starts_with(':') {
        return;
    }

    let (name, value) = match line.find(':') {
        Some(colon) => {
            let mut value = &line[colon + 1..];
            if value.starts_with(' ') {
                value = &value[1..];
            }
            (&line[..colon], value)
        }
        None => (line, ""),
    };

    match name {
        "event" => *kind = if value.is_empty() { None } else { Some(value.to_string()) },
        "data" => {
            data.push_str(value);
            data.push('\n');
            *has_data = true;
        }
        "id" => *id = Some(value.to_string()),
        _ => {}
    }
}
