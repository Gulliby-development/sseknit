mod common;

use common::row;

#[test]
fn rows() {
    row(&["text"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "Hello world\n", "");
    row(&["text"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\ndata: [DONE]\n\ndata: {\"delta\":\"never\"}\n\n", 0, "Hello world\n", "");
    row(&["text"], "", 0, "\n", "");
    row(&["text"], "data: {\"x\":1}\n\ndata: {\"delta\":\"Hel\"}\n\n", 0, "Hel\n", "");
    row(&["text", "--field", "x"], "data: {\"x\":\"X\"}\n\ndata: {\"delta\":\"d\"}\n\n", 0, "X\n", "");
    row(&["text", "--event", "chunk"], "event: chunk\ndata: {\"delta\":\"c\"}\n\ndata: {\"delta\":\"Hel\"}\n\n", 0, "c\n", "");
    row(&["text"], "event: ping\ndata: not json\n\ndata: {\"delta\":\"Hel\"}\n\n", 0, "Hel\n", "");
    row(&["text"], "data: {\"delta\":\"Hel\"}\n\nevent: error\ndata: overloaded\n\ndata: {\"delta\":\"lo\"}\n\n", 1, "", "sseknit: event 2: stream error: overloaded\n");
    row(&["text"], "data: {\"delta\":1}\n\n", 1, "", "sseknit: event 1: field delta is not a string\n");
    row(&["text"], "data: {\"delta\":null}\n\n", 1, "", "sseknit: event 1: field delta is not a string\n");
    row(&["text"], "data: nope\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {\"delta\":\"Hel\"}\n\ndata: [1]\n\n", 1, "", "sseknit: event 2: invalid JSON\n");
    row(&["text", "--event", "error"], "event: error\ndata: e\n\n", 1, "", "sseknit: event 1: stream error: e\n");
    row(&["text"], "data: {\"delta\":\"\"}\n\n", 0, "\n", "");
    row(&["text"], "data: {\"delta\":\"a\"}\ndata: {\"delta\":\"b\"}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
}
