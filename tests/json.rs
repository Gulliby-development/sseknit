mod common;

use common::row;

#[test]
fn rows() {
    row(&["text"], "data: {\"delta\":\"a\\nb\"}\n\n", 0, "a\nb\n", "");
    row(&["text"], "data: {\"delta\":\"a\\tb\\r\"}\n\n", 0, "a\tb\r\n", "");
    row(&["text"], "data: {\"delta\":\"q\\\"q\"}\n\n", 0, "q\"q\n", "");
    row(&["text"], "data: {\"delta\":\"back\\\\slash\"}\n\n", 0, "back\\slash\n", "");
    row(&["text"], "data: {\"delta\":\"sl\\/ash\"}\n\n", 0, "sl/ash\n", "");
    row(&["text"], "data: {\"delta\":\"\\u0041\\u007e\"}\n\n", 0, "A~\n", "");
    row(&["text"], "data: {\"delta\":\"\\u00e9\"}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {\"delta\":\"\\x41\"}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {\"delta\":\"\\u12\"}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {\"delta\":\"a\tb\"}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data:  { \"delta\" : \"sp\" , \"n\" : 1 }  \n\n", 0, "sp\n", "");
    row(&["text"], "data: {\"delta\":\"a\"} x\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {\"delta\":{\"a\":1}}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {\"delta\":\"a\",\"delta\":\"b\"}\n\n", 0, "b\n", "");
    row(&["text"], "data: {\"delta\":\"a\",}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text"], "data: {}\n\n", 0, "\n", "");
    row(&["text", "--field", "n"], "data: {\"n\":01}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text", "--field", "n"], "data: {\"n\":-}\n\n", 1, "", "sseknit: event 1: invalid JSON\n");
    row(&["text", "--field", "s"], "data: {\"n\":1.5e3,\"b\":true,\"z\":null,\"s\":\"ok\"}\n\n", 0, "ok\n", "");
}
