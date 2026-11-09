use crate::{Call, Failure, Value};

/// Aggregate the last prompt and completion token counts across all stream events.
pub fn usage(_call: &Call, input: &str) -> Result<String, Failure> {
    let mut prompt = 0;
    let mut completion = 0;

    for (index, event) in crate::sse::parse(input).iter().enumerate() {
        if event.data == "[DONE]" {
            continue;
        }
        let Some(members) = crate::json::parse_flat(&event.data) else {
            return Err(Failure::new(1, format!("event {}: invalid JSON", index + 1)));
        };
        prompt = token_value(&members, "prompt_tokens", index + 1)?.unwrap_or(prompt);
        completion = token_value(&members, "completion_tokens", index + 1)?.unwrap_or(completion);
    }

    Ok(format!("prompt {prompt}\ncompletion {completion}\ntotal {}\n", prompt + completion))
}

/// Summarize timed events, their first text latency, and adjacent timestamp gaps.
pub fn timeline(_call: &Call, input: &str) -> Result<String, Failure> {
    let mut count = 0;
    let mut first = 0;
    let mut previous = 0;
    let mut last = 0;
    let mut ttft = None;
    let mut max_gap = 0;

    for (index, event) in crate::sse::parse(input).iter().enumerate() {
        let Some(members) = crate::json::parse_flat(&event.data) else {
            continue;
        };
        let Some(Value::Int(timestamp)) = crate::json::get(&members, "ms") else {
            continue;
        };
        if *timestamp < 0 {
            continue;
        }

        if count > 0 && *timestamp < previous {
            return Err(Failure::new(1, format!("event {}: ms goes back from {previous} to {timestamp}", index + 1)));
        }
        if count == 0 {
            first = *timestamp;
        } else {
            max_gap = max_gap.max(*timestamp - previous);
        }
        count += 1;
        previous = *timestamp;
        last = *timestamp;

        if ttft.is_none() && matches!(crate::json::get(&members, "delta"), Some(Value::Str(text)) if !text.is_empty()) {
            ttft = Some(*timestamp - first);
        }
    }

    if count == 0 {
        return Ok("events 0\n".to_string());
    }
    Ok(format!("events {count}\nfirst {first}\nlast {last}\nttft {}\nmaxgap {max_gap}\nduration {}\n", ttft.map_or_else(|| "-".to_string(), |value| value.to_string()), last - first))
}

fn token_value(members: &[(String, Value)], name: &str, event_number: usize) -> Result<Option<i64>, Failure> {
    match crate::json::get(members, name) {
        None => Ok(None),
        Some(Value::Int(value)) if *value >= 0 => Ok(Some(*value)),
        Some(_) => Err(Failure::new(1, format!("event {event_number}: field {name} is not an integer"))),
    }
}
