<p align="center"><img src="docs/logo.svg" width="96" alt="" /></p>

# sseknit

[![CI](https://github.com/Gulliby-development/sseknit/actions/workflows/ci.yml/badge.svg)](https://github.com/Gulliby-development/sseknit/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

sseknit is a small Rust 2021 command-line utility for inspecting and assembling model output saved as a server-sent events (SSE) stream. It is intended for people building or debugging LLM clients, proxies, and evaluation harnesses: it turns fragmented text, tool-call arguments, usage counts, and timing data into readable output.

The program uses only Rust's standard library and is designed to build with stable Rust on Windows, macOS, and Linux. The examples below use `printf` and a POSIX-compatible shell; the Rust program itself does not depend on that shell.

## Build and run

Build from the repository root:

```sh
cargo build
```

To assemble text from an SSE file:

```sh
cargo run -- text < answer.sse
```

The command reads standard input as one complete stream. Successful commands write their result to standard output. A failure writes one diagnostic line to standard error and does not leave partial output on standard output.

## Command synopsis

```text
usage: sseknit events [--type T]
       sseknit text [--field F] [--event T]
       sseknit stop [--field F] [--event T] STOP...
       sseknit usage
       sseknit tools
       sseknit timeline
       sseknit rechunk N
```

The first word may be `help`, `-h`, or `--help` to print usage, or `--version` to print `sseknit 0.1.0`; these invocations ignore any following words and do not read standard input. With no command, usage is printed to standard error and the process exits with status 2 without reading standard input. Command names and options are case-sensitive. Successful commands exit 0, write their result to standard output, and leave standard error empty. Arguments after a command are checked from left to right before the stream is processed. A word beginning with `-` and containing more than one character is treated as an option; other words, including `-`, are positional. An option value is the next word even if it begins with `-`. Options must be supported and unique, and must have a non-empty value. `rechunk N` accepts one to five decimal digits, including leading zeros, whose value is 1 through 200.

### Worked examples

Each example supplies a small SSE stream inline. The displayed output is exact. These `printf` commands require a compatible shell; the Rust program itself is portable.

**events** lists events with their original 1-based numbers, type, ID and data. `--type T` selects one type without renumbering.

```sh
printf 'event: chunk\ndata: hello\n\n' | cargo run --quiet -- events
```

```text
1 chunk id=- data=hello
```

**text** joins the selected text fragments. By default it reads `delta` fields from `message` events.

```sh
printf 'data: {"delta":"Hi"}\n\ndata: {"delta":"!"}\n\n' | cargo run --quiet -- text
```

```text
Hi!
```

**stop** assembles text as `text` does and cuts at the earliest occurrence of any supplied stop sequence. If matches start at the same character, the longest wins.

```sh
printf 'data: {"delta":"Hello STOP"}\n\n' | cargo run --quiet -- stop STOP
```

```text
Hello 
-- stopped by [STOP] at offset 6 in event 1
```

**usage** reports the last prompt and completion token counts seen; absent counts are zero.

```sh
printf 'data: {"prompt_tokens":8,"completion_tokens":3}\n\n' | cargo run --quiet -- usage
```

```text
prompt 8
completion 3
total 11
```

**tools** assembles indexed tool-call fragments from message events, sorts calls by index, and reports whether their arguments have balanced outer structure.

```sh
printf 'data: {"index":0,"name":"lookup","args":"{\\"q\\":1}"}\n\n' | cargo run --quiet -- tools
```

```text
call 0 lookup complete {"q":1}
```

**timeline** reports timing metrics for valid events with a non-negative integer `ms` field.

```sh
printf 'data: {"ms":100,"delta":"Hi"}\n\ndata: {"ms":125}\n\n' | cargo run --quiet -- timeline
```

```text
events 2
first 100
last 125
ttft 0
maxgap 25
duration 25
```

**rechunk N** assembles the default text stream and writes it back as SSE pieces of N Unicode scalar values, followed by a done event. N must be from 1 through 200.

```sh
printf 'data: {"delta":"Hello"}\n\n' | cargo run --quiet -- rechunk 3
```

```text
data: {"delta":"Hel"}

data: {"delta":"lo"}

data: [DONE]

```

## Stream and data rules

Input is split into lines at LF, CR, or CRLF (CRLF is one line ending). An empty line closes a block. Lines beginning with a colon are comments. Other lines are fields: the name is before the first colon, and the value is after it with at most one leading space removed. A field without a colon has an empty value.

A block with one or more `data` fields creates one event; those values are joined with LF. Event type defaults to `message`, and the latest `event` field in the block sets it. IDs persist across events until another `id` field changes them, including an empty ID. Other fields, including `retry`, are ignored. A block without data creates no event, and an unclosed final block is discarded.

Commands that interpret event data as JSON accept only a flat JSON object: string keys and scalar values (strings, integers, other JSON numbers, booleans, or null). Nested arrays and objects, trailing commas, leading-zero integers, and extra text are invalid. JSON strings cannot contain raw control characters. Standard JSON string escapes are accepted; `\\uXXXX` escapes are limited to ASCII codes below 128. Integers have at most 15 digits. When a key occurs more than once, its last value is used.

### Command behavior

- **events** renders each event as `N TYPE id=ID data=DATA`, where ID is `-` if absent or empty. Data escapes backslash, LF, and tab as `\\`, `\n`, and `\t`; optional type filtering preserves original event numbers.
- **text** checks an `error` event before applying the event-type filter, and fails with its event number and data. Other unselected types are skipped. A selected event whose data is exactly `[DONE]` ends processing; other selected event data must be valid flat JSON. If the configured field exists, it must be a string. Fragments are concatenated and one LF is appended to the result.
- **stop** validates and assembles the selected stream just like `text` before searching. Thus an error in the selected stream is reported even if a stop match appeared earlier. It chooses the earliest match, then the longest at the same start. Its second line is `-- stopped by [STOP] at offset OFFSET in event N`, or `-- no stop sequence matched`; offsets count Unicode scalar values and N is the event containing the match's last character.
- **usage** scans every event type; exact `[DONE]` records are skipped but do not stop the scan. Other records must be valid flat JSON. Non-negative integer `prompt_tokens` and `completion_tokens` values update the respective totals, with the last value winning. It prints `prompt P`, `completion C`, and `total T` on separate lines.
- **tools** processes message events, fails on stream error, and stops at `[DONE]`. Objects without an `index` are skipped; an index must be a non-negative integer, and names and argument fragments, when present, must be strings. Each index keeps its first non-empty name and joins argument fragments in order; a conflicting later name is an error. Calls are sorted and rendered as `call I NAME STATUS ARGS`, using `-` for a missing name. The separating space and ARGS are omitted when arguments are empty. STATUS is `complete` only when the trimmed arguments start with `{` or `[` and all braces/brackets are closed in matching order outside strings; a backslash inside a string escapes the next character. This is a structural check, not JSON validation.
- **timeline** considers every event with a valid flat JSON object containing a non-negative integer `ms`; other records are ignored. Decreasing timestamps fail. With no timed events it prints only `events 0`. Otherwise it prints `events N`, `first F`, `last L`, `ttft T`, `maxgap G`, and `duration D`. TTFT is the first timed event's non-empty string `delta` timestamp minus F, or `-` when absent; maxgap is the greatest adjacent timestamp difference (0 for one event), and duration is L minus F.
- **rechunk** uses default `text` selection and the same stream errors. It splits assembled text by Unicode scalar value, escapes `\\` as `\\\\`, quote as `\\\"`, LF/CR/tab as `\\n`/`\\r`/`\\t`, and any other character below 32 as `\\u` plus four lowercase hexadecimal digits; other characters pass through. Each chunk is a `data: {"delta":"PIECE"}` event, followed by a final `[DONE]` event. Empty text produces only that final event.

## Errors and exit status

A command-line failure exits with status 2. Common exact diagnostics are:

- `sseknit: unknown command WORD`
- `sseknit: unknown option WORD`
- `sseknit: WORD given twice`
- `sseknit: WORD needs a non-empty name`
- `sseknit: unexpected argument WORD`
- `sseknit: stop needs STOP...`
- `sseknit: stop needs non-empty stop sequences`
- `sseknit: rechunk needs N`
- `sseknit: rechunk needs an integer from 1 to 200`

Stream-processing failures exit with status 1. Their diagnostics begin with `sseknit: event N:` and include the applicable reason, such as `invalid JSON`, `stream error: DATA`, `field NAME is not a string`, `field NAME is not an integer`, `field index is not a non-negative integer`, `tool I renamed from OLD to NEW`, or `ms goes back from OLD to NEW`. If standard input cannot be read, the diagnostic is `sseknit: cannot read standard input`. Failures produce no standard output; diagnostic lines end in LF.

## Development

Run the project checks from the repository root:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

The project is distributed under the MIT License; see [LICENSE](LICENSE).

---

Built with [BAKED](https://github.com/Gulliby-development/baked), an open-source coding agent (bench round R84).
