# 4=10 Solver

This is a small command line tool for solving (a slightly generalized version of) game '4=10' by Sveinn Steinarsson.

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

If you're using linux