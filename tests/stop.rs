mod common;

use common::row;

#[test]
fn rows() {
    row(&["stop", "lo"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "Hel\n-- stopped by [lo] at offset 3 in event 2\n", "");
    row(&["stop", "xyz"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "Hello world\n-- no stop sequence matched\n", "");
    row(&["stop", "l", "wor"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "He\n-- stopped by [l] at offset 2 in event 1\n", "");
    row(&["stop", "o w"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "Hell\n-- stopped by [o w] at offset 4 in event 3\n", "");
    row(&["stop", "Hello world"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "\n-- stopped by [Hello world] at offset 0 in event 3\n", "");
    row(&["stop", "H"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "\n-- stopped by [H] at offset 0 in event 1\n", "");
    row(&["stop", "ld", "orld"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "Hello w\n-- stopped by [orld] at offset 7 in event 3\n", "");
    row(&["stop", "l", "ll"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "He\n-- stopped by [ll] at offset 2 in event 2\n", "");
    row(&["stop", "d"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\ndata: [DONE]\n\ndata: {\"delta\":\"d\"}\n\n", 0, "Hello worl\n-- stopped by [d] at offset 10 in event 3\n", "");
    row(&["stop", "a"], "", 0, "\n-- no stop sequence matched\n", "");
    row(&["stop", "x", "--field", "t"], "data: {\"t\":\"abx\"}\n\n", 0, "ab\n-- stopped by [x] at offset 2 in event 1\n", "");
    row(&["stop", "b"], "data: {\"delta\":\"ab\"}\n\ndata: {\"delta\":\"b\"}\n\n", 0, "a\n-- stopped by [b] at offset 1 in event 1\n", "");
    row(&["stop", "e\\nf"], "data: {\"delta\":\"e\\nf\"}\n\n", 0, "e\nf\n-- no stop sequence matched\n", "");
    row(&["stop", "e", "--event", "m"], "event: m\ndata: {\"delta\":\"ee\"}\n\n", 0, "\n-- stopped by [e] at offset 0 in event 1\n", "");
}
