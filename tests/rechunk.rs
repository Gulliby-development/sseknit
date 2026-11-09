mod common;

use common::row;

#[test]
fn rows() {
    row(&["rechunk", "3"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo \"}\n\ndata: {\"delta\":\"wor\"}\n\ndata: {\"delta\":\"ld\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "1"], "data: {\"delta\":\"Hel\"}\n\n", 0, "data: {\"delta\":\"H\"}\n\ndata: {\"delta\":\"e\"}\n\ndata: {\"delta\":\"l\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "200"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "data: {\"delta\":\"Hello world\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "2"], "", 0, "data: [DONE]\n\n", "");
    row(&["rechunk", "4"], "data: {\"delta\":\"a\\nb\\t\\\"\\\\\"}\n\n", 0, "data: {\"delta\":\"a\\nb\\t\"}\n\ndata: {\"delta\":\"\\\"\\\\\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "2"], "data: {\"delta\":\"\\u0001\\u001f\"}\n\n", 0, "data: {\"delta\":\"\\u0001\\u001f\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "007"], "data: {\"delta\":\"abcdefghijklmnop\"}\n\n", 0, "data: {\"delta\":\"abcdefg\"}\n\ndata: {\"delta\":\"hijklmn\"}\n\ndata: {\"delta\":\"op\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "5"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\ndata: [DONE]\n\ndata: {\"delta\":\"tail\"}\n\n", 0, "data: {\"delta\":\"Hello\"}\n\ndata: {\"delta\":\" worl\"}\n\ndata: {\"delta\":\"d\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "5"], "event: x\ndata: {\"delta\":\"skipped\"}\n\ndata: {\"delta\":\"Hel\"}\n\n", 0, "data: {\"delta\":\"Hel\"}\n\ndata: [DONE]\n\n", "");
    row(&["rechunk", "5"], "data: {\"delta\":1}\n\n", 1, "", "sseknit: event 1: field delta is not a string\n");
    row(&["rechunk", "5"], "event: error\ndata: bad\n\n", 1, "", "sseknit: event 1: stream error: bad\n");
    row(&["rechunk", "3"], "data: {\"x\":1}\n\n", 0, "data: [DONE]\n\n", "");
}
