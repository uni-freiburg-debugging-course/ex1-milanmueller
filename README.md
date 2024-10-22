[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/3MmVbb7f)
# Debugging 24/25 Exercise 1

Detailed instructions are in the exercise sheet. Following are your checkpoints:

- [x] Implement parser and evaluator
- [ ] Implement a fuzzer
- [ ] Generate *lots* of random instances with your fuzzer. Your evaluator and z3 must return the same result on generated instances
- [ ] Provide detailed build instructions for your code so that we can evaluate it

# Dependencies for Building the Project
The project written in rust, and building is done using Cargo.
I installed cargo using [rustup](https://rustup.rs/), but I assume installing rust and cargo through a package manager would also work.

# Build Instructions
To build this project, run `cargo build --release`. This creates two executables
* `./target/release/parser` - for the parser
* `./target/release/fuzzer` - for the fuzzer

# Usage
The fuzzer
