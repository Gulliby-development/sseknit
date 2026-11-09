mod common;

use common::row;

#[test]
fn rows() {
    row(&["tools"], "data: {\"index\":0,\"name\":\"a\"}\n\ndata: {\"index\":0,\"name\":\"b\"}\n\n", 1, "", "sseknit: event 2: tool 0 renamed from a to b\n");
    row(&["tools"], "data: {\"index\":-1}\n\n", 1, "", "sseknit: event 1: field index is not a non-negative integer\n");
    row(&["tools"], "data: {\"index\":\"0\"}\n\n", 1, "", "sseknit: event 1: field index is not a non-negative integer\n");
    row(&["tools"], "data: {\"index\":1.5}\n\n", 1, "", "sseknit: event 1: field index is not a non-negative integer\n");
    row(&["tools"], "data: {\"index\":0,\"name\":3}\n\n", 1, "", "sseknit: event 1: field name is not a string\n");
    row(&["tools"], "data: {\"index\":0,\"args\":null}\n\n", 1, "", "sseknit: event 1: field args is not a string\n");
    row(&["tools"], "data: {\"index\":0}\n\ndata: oops\n\n", 1, "", "sseknit: event 2: invalid JSON\n");
    row(&["tools"], "data: {\"index\":0}\n\nevent: error\ndata: bad\n\n", 1, "", "sseknit: event 2: stream error: bad\n");
    row(&["tools"], "data: {\"index\":1234567890123456}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["tools"], "data: {\"index\":123456789012345,\"name\":\"big\"}\n\n", 0, "call 123456789012345 big incomplete\n", "");
}
