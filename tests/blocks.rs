mod common;

use common::row;

#[test]
fn rows() {
    row(&["events"], "event: ping\ndata: x\n\ndata: y\n\n", 0, "1 ping id=- data=x\n2 message id=- data=y\n", "");
    row(&["events"], "event:\ndata: x\n\n", 0, "1 message id=- data=x\n", "");
    row(&["events"], "id: 7\ndata: a\n\ndata: b\n\n", 0, "1 message id=7 data=a\n2 message id=7 data=b\n", "");
    row(&["events"], "id: 7\ndata: a\n\nid:\ndata: b\n\n", 0, "1 message id=7 data=a\n2 message id=- data=b\n", "");
    row(&["events"], "id\ndata: a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "retry: 100\ndata: a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "foo: bar\ndata: a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "event: ping\n\ndata: a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "Event: ping\nDATA: a\n\ndata: b\n\n", 0, "1 message id=- data=b\n", "");
    row(&["events"], "id: 1\ndata: a\n\nid: 2\ndata: b\n\ndata: c\n\n", 0, "1 message id=1 data=a\n2 message id=2 data=b\n3 message id=2 data=c\n", "");
    row(&["events"], "data:\n\n", 0, "1 message id=- data=\n", "");
    row(&["events"], "event: a\nevent: b\ndata: x\n\n", 0, "1 b id=- data=x\n", "");
    row(&["events"], "id: 5\nevent: e\ndata: x\ny\n\n", 0, "1 e id=5 data=x\n", "");
}
