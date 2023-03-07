# Turing Machine Simulator

The Turing Machine simulator is a simulator for Turing Machines. It was written in [Rust](https://rust-lang.org) using [Pest](https://pest.rs) and uses no nightly features.

## Usage

The program can be used as follows (assuming your compiled binary is called `tm`):

```bash
tm <program> <input> [default = "#"]
```

For example:

```bash
tm unary-add.tm "1111#11"
# gives output 111111
tm unary-sub.tm "000011" "#"
# gives output 00
```

## Input format

Input files are written to form a table of rules.

Comments can be added to the file using syntax typical to that of C-like languages. Please note that, like Rust, multi-line comments *may* be nested, and therefore do not behave like many other C-like languages (where any further `/*` inside a multi-line comment are ignored).

The first column represents the state name, which can be any string not containing white-space or reserved characters.

The rest of the line contains the transition rules. These are represented as three-to-four tuples, separated by slashes. The first segment denotes the value being read from the tape, the second segment denotes the value being written to the tape, the third segment denotes the desired action, and the fourth segment, if present, denotes the state to transition to.

### Action

The action performed by the Turing machine during a given transaction may be one of the following:

- `<`: Move the head left
- `>`: Move the head right
- `=`: Accept the input
- `!`: Reject the input
- `?`: Return the current tape state
