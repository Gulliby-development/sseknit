mod common;

use common::row;

#[test]
fn rows() {
    row(&["usage"], "data: {\"prompt_tokens\":10}\n\ndata: {\"completion_tokens\":5}\n\n", 0, "prompt 10\ncompletion 5\ntotal 15\n", "");
    row(&["usage"], "", 0, "prompt 0\ncompletion 0\ntotal 0\n", "");
    row(&["usage"], "data: {\"prompt_tokens\":10,\"completion_tokens\":1}\n\ndata: {\"prompt_tokens\":10,\"completion_tokens\":7}\n\n", 0, "prompt 10\ncompletion 7\ntotal 17\n", "");
    row(&["usage"], "data: {\"delta\":\"Hel\"}\n\ndata: {\"delta\":\"lo\"}\n\ndata: {\"delta\":\" world\"}\n\ndata: [DONE]\n\n", 0, "prompt 0\ncompletion 0\ntotal 0\n", "");
    row(&["usage"], "data: {\"prompt_tokens\":\"10\"}\n\n", 1, "", "sseknit: event 1: field prompt_tokens is not an integer\n");
    row(&["usage"], "data: {\"completion_tokens\":-3}\n\n", 1, "", "sseknit: event 1: field completion_tokens is not an integer\n");
    row(&["usage"], "data: {\"completion_tokens\":2.5}\n\n", 1, "", "sseknit: event 1: field completion_tokens is not an integer\n");
    row(&["usage"], "data: {\"prompt_tokens\":null}\n\n", 1, "", "sseknit: event 1: field prompt_tokens is not an integer\n");
    row(&["usage"], "event: usage\ndata: {\"prompt_tokens\":4,\"completion_tokens\":6}\n\n", 0, "prompt 4\ncompletion 6\ntotal 10\n", "");
    row(&["usage"], "data: {\"prompt_tokens\":1}\n\ndata: broken\n\n", 1, "", "sseknit: event 2: invalid JSON\n");
    row(&["usage"], "data: {\"prompt_tokens\":4}\n\ndata: [DONE]\n\ndata: {\"completion_tokens\":9}\n\n", 0, "prompt 4\ncompletion 9\ntotal 13\n", "");
}
