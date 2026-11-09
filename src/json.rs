use crate::Value;

/// Parse one complete flat JSON object, preserving member order and duplicate keys.
pub fn parse_flat(text: &str) -> Option<Vec<(String, Value)>> {
    let mut parser = Parser { text, pos: 0 };
    parser.whitespace();
    parser.expect(b'{')?;
    parser.whitespace();

    let mut members = Vec::new();
    if parser.consume(b'}') {
        parser.whitespace();
        return (parser.pos == text.len()).then_some(members);
    }

    loop {
        parser.whitespace();
        let key = parser.string()?;
        parser.whitespace();
        parser.expect(b':')?;
        parser.whitespace();
        let value = parser.value()?;
        members.push((key, value));
        parser.whitespace();
        if parser.consume(b'}') {
            break;
        }
        parser.expect(b',')?;
        parser.whitespace();
    }

    parser.whitespace();
    (parser.pos == text.len()).then_some(members)
}

/// Return the value belonging to the last matching member.
pub fn get<'a>(members: &'a [(String, Value)], key: &str) -> Option<&'a Value> {
    members.iter().rev().find(|(member, _)| member == key).map(|(_, value)| value)
}

/// Escape text as the body of a JSON string, without surrounding quote marks.
pub fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if (ch as u32) < 0x20 => {
                use std::fmt::Write;
                let _ = write!(escaped, "\\u{:04x}", ch as u32);
            }
            ch => escaped.push(ch),
        }
    }
    escaped
}

struct Parser<'a> {
    text: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.text.as_bytes().get(self.pos).copied()
    }

    fn consume(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, byte: u8) -> Option<()> {
        self.consume(byte).then_some(())
    }

    fn string(&mut self) -> Option<String> {
        self.expect(b'"')?;
        let mut value = String::new();
        loop {
            let byte = self.peek()?;
            match byte {
                b'"' => {
                    self.pos += 1;
                    return Some(value);
                }
                0..=0x1f => return None,
                b'\\' => {
                    self.pos += 1;
                    let escaped = self.peek()?;
                    self.pos += 1;
                    match escaped {
                        b'"' => value.push('"'),
                        b'\\' => value.push('\\'),
                        b'/' => value.push('/'),
                        b'b' => value.push('\u{0008}'),
                        b'f' => value.push('\u{000c}'),
                        b'n' => value.push('\n'),
                        b'r' => value.push('\r'),
                        b't' => value.push('\t'),
                        b'u' => {
                            let mut code = 0u32;
                            for _ in 0..4 {
                                let digit = self.peek()?;
                                self.pos += 1;
                                code = code.checked_mul(16)?.checked_add(hex(digit)? as u32)?;
                            }
                            if code >= 128 {
                                return None;
                            }
                            value.push(char::from_u32(code)?);
                        }
                        _ => return None,
                    }
                }
                _ => {
                    let ch = self.text[self.pos..].chars().next()?;
                    value.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
    }

    fn value(&mut self) -> Option<Value> {
        match self.peek()? {
            b'"' => self.string().map(Value::Str),
            b't' => {
                self.literal(b"true")?;
                Some(Value::Bool(true))
            }
            b'f' => {
                self.literal(b"false")?;
                Some(Value::Bool(false))
            }
            b'n' => {
                self.literal(b"null")?;
                Some(Value::Null)
            }
            b'-' | b'0'..=b'9' => self.number(),
            _ => None,
        }
    }

    fn literal(&mut self, literal: &[u8]) -> Option<()> {
        let end = self.pos.checked_add(literal.len())?;
        if self.text.as_bytes().get(self.pos..end)? != literal {
            return None;
        }
        self.pos = end;
        Some(())
    }

    fn number(&mut self) -> Option<Value> {
        let start = self.pos;
        self.consume(b'-');
        let integer_start = self.pos;
        match self.peek()? {
            b'0' => {
                self.pos += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return None;
                }
            }
            b'1'..=b'9' => {
                self.pos += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return None,
        }

        let integer_digits = self.pos - integer_start;
        let mut non_integer = false;
        if self.consume(b'.') {
            non_integer = true;
            let fraction_start = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if self.pos == fraction_start {
                return None;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            non_integer = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            let exponent_start = self.pos;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
            if self.pos == exponent_start {
                return None;
            }
        }

        if non_integer {
            Some(Value::Num)
        } else if integer_digits <= 15 {
            let integer = self.text.get(start..self.pos)?.parse::<i64>().ok()?;
            Some(Value::Int(integer))
        } else {
            None
        }
    }
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
