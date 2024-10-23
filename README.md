[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/3MmVbb7f)
# Debugging 24/25 Exercise 1

Detailed instructions are in the exercise sheet. Following are your checkpoints:

- [x] Implement parser and evaluator
- [x] Implement a fuzzer
- [x] Generate *lots* of random instances with your fuzzer. Your evaluator and z3 must return the same result on generated instances
- [x] Provide detailed build instructions for your code so that we can evaluate it

# Dependencies for Building the Project
The project written in rust, and building is done using Cargo.
I installed cargo using [rustup](https://rustup.rs/), but I assume installing rust and cargo through a package manager would also work.

# Build Instructions
To build this project, run `cargo build --release`. This creates two executables
* `./target/release/parser` - for the parser
* `./target/release/fuzzer` - for the fuzzer

Alternatively, the parser and fuzzer can also be run directly with cargo, using
* `cargo run --release --bin parser -- input.smt2` - for the parser
* `cargo run --release --bin fuzzer` - for the fuzzer

# Usage
## Parser
The parser expects an SMT2 file as input. It will then parse and evaluate the file.
For now, evaluating means that operators will be evaluated if enclosed in a `simplify` command.
As an alternative to using a file as input, the parser can also parse and evaluate input from stdin, if the flag `--stdin` is set.

## Fuzzer
The fuzzer generates random smt clauses in the form
`(simplify (op a b))`, where op is one of `+`, `-`, `*` and `a` and `b` are numbers
such that the result of the operation is within the range of a 32-bit signed integer.
If called without any arguments, the fuzzer will generate one clause.
With the flag `--number` a specific number of clauses can be generated.
