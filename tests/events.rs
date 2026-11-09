mod common;

use common::row;

#[test]
fn rows() {
    row(&["events", "--type", "message"], "event: a\ndata: 1\n\ndata: 2\n\nevent: a\ndata: 3\n\n", 0, "2 message id=- data=2\n", "");
    row(&["events", "--type", "a"], "event: a\ndata: 1\n\ndata: 2\n\nevent: a\ndata: 3\n\n", 0, "1 a id=- data=1\n3 a id=- data=3\n", "");
    row(&["events", "--type", "none"], "data: 1\n\n", 0, "", "");
    row(&["events", "--type", "Message"], "data: 1\n\n", 0, "", "");
    row(&["events"], "event: error\ndata: boom\n\n", 0, "1 error id=- data=boom\n", "");
    row(&["events"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\ndata: [DONE]\n\n", 0, "1 message id=- data={\"delta\":\"Hel\"}\n2 message id=- data={\"delta\":\"lo\"}\n3 message id=- data={\"delta\":\" world\"}\n4 message id=- data=[DONE]\n", "");
    row(&["events"], "id: 9\ndata: line1\ndata: line2\n\nevent: x\ndata: back\\slash\n\n", 0, "1 message id=9 data=line1\\nline2\n2 x id=9 data=back\\\\slash\n", "");
    row(&["events", "--type", "x", "--type", "y"], "", 2, "", "sseknit: --type given twice\n");
    row(&["events"], "data: {\"a\":\"b\"}\n\n", 0, "1 message id=- data={\"a\":\"b\"}\n", "");
    row(&["events"], "id: a b\nevent: t t\ndata: d d\n\n", 0, "1 t t id=a b data=d d\n", "");
}
