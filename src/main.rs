use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input = String::new();
    if sseknit::reads_stdin(&args) {
        let mut bytes = Vec::new();
        if std::io::stdin().read_to_end(&mut bytes).is_err() {
            eprintln!("sseknit: cannot read standard input");
            std::process::exit(1);
        }
        input = String::from_utf8_lossy(&bytes).into_owned();
    }
    let outcome = sseknit::run(&args, &input);
    let _ = std::io::stdout().write_all(outcome.out.as_bytes());
    let _ = std::io::stderr().write_all(outcome.err.as_bytes());
    std::process::exit(outcome.code);
}
