use sseknit::run;

/// Runs one command line against the program and compares the exit status and both output strings.
pub fn row(args: &[&str], input: &str, code: i32, out: &str, err: &str) {
    let words: Vec<String> = args.iter().map(|word| word.to_string()).collect();
    let got = run(&words, input);
    assert_eq!((got.code, got.out.as_str(), got.err.as_str()), (code, out, err), "sseknit {args:?} with input {input:?}");
}
