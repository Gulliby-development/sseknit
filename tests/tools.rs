mod common;

use common::row;

#[test]
fn rows() {
    row(&["tools"], "data: {\"index\":0,\"name\":\"get\",\"args\":\"{\\\"a\\\":1}\"}\n\n", 0, "call 0 get complete {\"a\":1}\n", "");
    row(&["tools"], "data: {\"index\":0,\"name\":\"get\",\"args\":\"{\\\"a\\\"\"}\n\ndata: {\"index\":0,\"args\":\":1}\"}\n\n", 0, "call 0 get complete {\"a\":1}\n", "");
    row(&["tools"], "data: {\"index\":1,\"name\":\"b\",\"args\":\"[1\"}\n\ndata: {\"index\":0,\"name\":\"a\",\"args\":\"{}\"}\n\n", 0, "call 0 a complete {}\ncall 1 b incomplete [1\n", "");
    row(&["tools"], "data: {\"index\":0,\"args\":\"{}\"}\n\n", 0, "call 0 - complete {}\n", "");
    row(&["tools"], "data: {\"index\":0,\"name\":\"n\"}\n\n", 0, "call 0 n incomplete\n", "");
    row(&["tools"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\n", 0, "", "");
    row(&["tools"], "", 0, "", "");
    row(&["tools"], "data: {\"index\":0,\"name\":\"a\"}\n\ndata: {\"index\":0,\"name\":\"a\",\"args\":\"[]\"}\n\n", 0, "call 0 a complete []\n", "");
    row(&["tools"], "data: {\"index\":0,\"args\":\"x\\ny\"}\n\n", 0, "call 0 - incomplete x\\ny\n", "");
    row(&["tools"], "data: {\"index\":0,\"args\":\"{\\\"s\\\":\\\"}\\\"\"}\n\n", 0, "call 0 - incomplete {\"s\":\"}\"\n", "");
    row(&["tools"], "data: {\"index\":0,\"args\":\" [ ] \"}\n\n", 0, "call 0 - complete  [ ] \n", "");
    row(&["tools"], "data: {\"index\":0,\"args\":\"{}}\"}\n\n", 0, "call 0 - incomplete {}}\n", "");
    row(&["tools"], "data: {\"index\":0,\"args\":\"{\\\"a\\\":1}\"}\n\ndata: [DONE]\n\ndata: {\"index\":1}\n\n", 0, "call 0 - complete {\"a\":1}\n", "");
    row(&["tools"], "event: ping\ndata: {\"index\":3,\"name\":\"p\"}\n\n", 0, "", "");
}
