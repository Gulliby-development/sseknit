mod common;

use common::row;

#[test]
fn rows() {
    row(&[], "", 2, "", "usage: sseknit events [--type T]\n       sseknit text [--field F] [--event T]\n       sseknit stop [--field F] [--event T] STOP...\n       sseknit usage\n       sseknit tools\n       sseknit timeline\n       sseknit rechunk N\n");
    row(&["help"], "", 0, "usage: sseknit events [--type T]\n       sseknit text [--field F] [--event T]\n       sseknit stop [--field F] [--event T] STOP...\n       sseknit usage\n       sseknit tools\n       sseknit timeline\n       sseknit rechunk N\n", "");
    row(&["-h", "text", "--bogus"], "", 0, "usage: sseknit events [--type T]\n       sseknit text [--field F] [--event T]\n       sseknit stop [--field F] [--event T] STOP...\n       sseknit usage\n       sseknit tools\n       sseknit timeline\n       sseknit rechunk N\n", "");
    row(&["--help"], "", 0, "usage: sseknit events [--type T]\n       sseknit text [--field F] [--event T]\n       sseknit stop [--field F] [--event T] STOP...\n       sseknit usage\n       sseknit tools\n       sseknit timeline\n       sseknit rechunk N\n", "");
    row(&["--version"], "", 0, "sseknit 0.1.0\n", "");
    row(&["--version", "x"], "", 0, "sseknit 0.1.0\n", "");
    row(&["frob"], "", 2, "", "sseknit: unknown command frob\n");
    row(&[""], "", 2, "", "sseknit: unknown command \n");
    row(&["Text"], "", 2, "", "sseknit: unknown command Text\n");
    row(&["--field", "delta"], "", 2, "", "sseknit: unknown command --field\n");
}
