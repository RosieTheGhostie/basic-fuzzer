# basic-fuzzer

A basic CLI fuzzer.

> [!WARNING]
> This is **not** intended for production. This is a toy program written for the express purpose of
> learning and improving as a software developer.

## Purpose

Put simply, fuzz testing (or fuzzing) is the act of feeding a large amount of bad/garbage data to
the target program with the hope of producing a failing state. Such a state typically occurs when
the program improperly handles an edge-case, usually resulting in a crash.

A more complete and accurate description can be found [here](https://en.wikipedia.org/wiki/Fuzzing).

## Installation

For the sake of simplicity, it is recommended to just download the release binary from the releases
page on GitHub.

## Usage

For detailed usage, run the following command in the same directory as 'basic-fuzzer':

```bash
./basic-fuzzer --help
```

This will tell you how to run the fuzzer on any given program. Then, once you have ran the fuzzer
and produced a crashing state, there will be two new files: 'input-xxxxxxxxxxxx' and
'args-xxxxxxxxxxxx' ('x' is a placeholder for an arbitrary hex digit). The input is the `stdin`
input encoded in UTF-8. The program arguments, however, are encoded in a more complex manner. The
file represents an array of UTF-8 length-encoded strings, where the first eight bytes of a string
represent its length (in little endian). For example, this:

```text
05 00 00 00 00 00 00 00
68 65 6c 6c 6f
05 00 00 00 00 00 00 00
77 6f 72 6c 64
```

represents `["hello", "world"]`.

## Limitiations

This is (formally) a generation-based, dumb, black-box testing tool that can be used for grey-box
testing if configured correctly. Compared to mature, production-grade fuzzers, it is closer to a
monkey in the famous
[infinite monkey theorem](https://en.wikipedia.org/wiki/Infinite_monkey_theorem) than an intelligent
fuzzer. This is for a few reasons:

1. This is a toy program written without the assistance of extensive security knowledge.
2. Writing something more advanced would require more information about the program being fuzzed.
3. Deadlines...

## Ethical Considerations

As with all security testing tools, users must act responsibly. The intended use of this program is
(or would be if it were intended for production) to find bugs or vulnerabilities in a program one
has been given explicit permission to test. This should **not** be used for denial of service (DoS)
attacks. In general, any use of this program that has the potential to directly affect machines the
user does not have an explicit right to is not allowed.
