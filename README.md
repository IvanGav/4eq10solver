# 4=10 Solver

This is a small command line tool for solving (a slightly generalized version of) game `4=10` by Sveinn Steinarsson.

Given 4 digits, operators +, -, * and / and one set of parentheses, make the equation to equal 10.

Command Line Arguments:
- -n - specify Number of numbers to read and use; default = 4
- -f - what number to Find; default = 10.0
- -o - what Operators are allowed; default = +-*/
  - use any string of +-\*/, representing corresponding allowed operators (e.g. '+', '+\*', '+-/')
- -h - display a Helpful usage guide

Example usage:
- `./solver -n 3 -f 5 -o +-*`
- (enter) `1`
- (enter) `2`
- (enter) `3`

# How to run it

You can download a compiled executable from a tab on the right (`Releases`). There should be one for Windows and one for Linux.

Or if you want to build it yourself:
- Open a terminal with `git` and `cargo` installed
- `git clone git@github.com:IvanGav/4eq10solver.git`
- `cd 4eq10solver`
- `cargo run`
  - If you want to build for release: `cargo build --release`
  - The executable file will be in `./target/release`

# Other

Made with Rust.

My code is terrible, so don't look >:(