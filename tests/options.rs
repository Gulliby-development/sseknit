mod common;

use common::row;

#[test]
fn rows() {
    row(&["text", "-x"], "", 2, "", "sseknit: unknown option -x\n");
    row(&["text", "--Field", "a"], "", 2, "", "sseknit: unknown option --Field\n");
    row(&["text", "--field", "a", "--field", "b"], "", 2, "", "sseknit: --field given twice\n");
    row(&["text", "--field"], "", 2, "", "sseknit: --field needs a non-empty name\n");
    row(&["text", "--field", ""], "", 2, "", "sseknit: --field needs a non-empty name\n");
    row(&["events", "--type"], "", 2, "", "sseknit: --type needs a non-empty name\n");
    row(&["events", "-"], "", 2, "", "sseknit: unexpected argument -\n");
    row(&["events", "extra"], "", 2, "", "sseknit: unexpected argument extra\n");
    row(&["stop"], "", 2, "", "sseknit: stop needs STOP...\n");
    row(&["stop", "--field", "a"], "", 2, "", "sseknit: stop needs STOP...\n");
    row(&["stop", "a", ""], "", 2, "", "sseknit: stop needs non-empty stop sequences\n");
    row(&["rechunk"], "", 2, "", "sseknit: rechunk needs N\n");
    row(&["rechunk", "2", "3"], "", 2, "", "sseknit: unexpected argument 3\n");
    row(&["rechunk", "0"], "", 2, "", "sseknit: rechunk needs an integer from 1 to 200\n");
    row(&["rechunk", "201"], "", 2, "", "sseknit: rechunk needs an integer from 1 to 200\n");
    row(&["rechunk", "x"], "", 2, "", "sseknit: rechunk needs an integer from 1 to 200\n");
    row(&["rechunk", "100000"], "", 2, "", "sseknit: rechunk needs an integer from 1 to 200\n");
    row(&["usage", "--type", "x"], "", 2, "", "sseknit: unknown option --type\n");
}
