mod common;

use common::row;

#[test]
fn rows() {
    row(&["timeline"], "data: {\"ms\":100,\"delta\":\"a\"}\n\ndata: {\"ms\":130,\"delta\":\"b\"}\n\ndata: {\"ms\":250}\n\n", 0, "events 3\nfirst 100\nlast 250\nttft 0\nmaxgap 120\nduration 150\n", "");
    row(&["timeline"], "", 0, "events 0\n", "");
    row(&["timeline"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "events 0\n", "");
    row(&["timeline"], "data: {\"ms\":5}\n\n", 0, "events 1\nfirst 5\nlast 5\nttft -\nmaxgap 0\nduration 0\n", "");
    row(&["timeline"], "data: {\"ms\":5}\n\ndata: {\"ms\":9,\"delta\":\"x\"}\n\n", 0, "events 2\nfirst 5\nlast 9\nttft 4\nmaxgap 4\nduration 4\n", "");
    row(&["timeline"], "data: {\"ms\":5,\"delta\":\"\"}\n\ndata: {\"ms\":9}\n\n", 0, "events 2\nfirst 5\nlast 9\nttft -\nmaxgap 4\nduration 4\n", "");
    row(&["timeline"], "data: {\"ms\":10}\n\ndata: {\"ms\":4}\n\n", 1, "", "sseknit: event 2: ms goes back from 10 to 4\n");
    row(&["timeline"], "data: {\"ms\":10}\n\ndata: {\"ms\":10,\"delta\":\"z\"}\n\n", 0, "events 2\nfirst 10\nlast 10\nttft 0\nmaxgap 0\nduration 0\n", "");
    row(&["timeline"], "data: {\"ms\":-1}\n\ndata: {\"ms\":1.5}\n\ndata: {\"ms\":\"7\"}\n\n", 0, "events 0\n", "");
    row(&["timeline"], "data: nope\n\ndata: {\"ms\":3}\n\ndata: [DONE]\n\n", 0, "events 1\nfirst 3\nlast 3\nttft -\nmaxgap 0\nduration 0\n", "");
    row(&["timeline"], "data: {\"ms\":1}\n\ndata: {\"ms\":8}\n\ndata: {\"ms\":9}\n\ndata: {\"ms\":30}\n\n", 0, "events 4\nfirst 1\nlast 30\nttft -\nmaxgap 21\nduration 29\n", "");
    row(&["timeline"], "event: tick\ndata: {\"ms\":2,\"delta\":7}\n\n", 0, "events 1\nfirst 2\nlast 2\nttft -\nmaxgap 0\nduration 0\n", "");
}
