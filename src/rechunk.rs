use crate::{Call, Failure};

/// Re-emit assembled text as JSON-escaped SSE data chunks, followed by the done event.
pub fn run(call: &Call, input: &str) -> Result<String, Failure> {
    let size = call.positional.first().and_then(|value| value.parse::<usize>().ok()).ok_or_else(|| Failure::new(2, "rechunk needs an integer from 1 to 200"))?;
    let mut text = crate::text::text(&Call::default(), input)?;
    text.pop();

    let mut output = String::new();
    let mut chunk = String::new();
    let mut characters = 0;
    for character in text.chars() {
        chunk.push(character);
        characters += 1;
        if characters == size {
            write_chunk(&mut output, &chunk);
            chunk.clear();
            characters = 0;
        }
    }
    if !chunk.is_empty() {
        write_chunk(&mut output, &chunk);
    }
    output.push_str("data: [DONE]\n\n");
    Ok(output)
}

fn write_chunk(output: &mut String, chunk: &str) {
    output.push_str("data: {\"delta\":\"");
    output.push_str(&crate::json::escape(chunk));
    output.push_str("\"}\n\n");
}
