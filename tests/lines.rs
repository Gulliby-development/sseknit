mod common;

use common::row;

#[test]
fn rows() {
    row(&["events"], "data: a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "data: a\r\n\r\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "data: a\r\r", 0, "1 message id=- data=a\n", "");
    row(&["events"], "data: a\n\r\ndata: b\r\n\n", 0, "1 message id=- data=a\n2 message id=- data=b\n", "");
    row(&["events"], ": comment\ndata: a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "data:a\n\n", 0, "1 message id=- data=a\n", "");
    row(&["events"], "data:  two\n\n", 0, "1 message id=- data= two\n", "");
    row(&["events"], "data: a\ndata: b\ndata\n\n", 0, "1 message id=- data=a\\nb\\n\n", "");
    row(&["events"], "data\n\n", 0, "1 message id=- data=\n", "");
    row(&["events"], "data: a\n", 0, "", "");
    row(&["events"], "data: a\n\ndata: b", 0, "1 message id=- data=a\n", "");
    row(&["events"], "", 0, "", "");
    row(&["events"], "\n\n\n", 0, "", "");
    row(&["events"], "data: tab\there\\n\n\n", 0, "1 message id=- data=tab\\there\\\\n\n", "");
    row(&["events"], "data: a:b: c\n\n", 0, "1 message id=- data=a:b: c\n", "");
}
