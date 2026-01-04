# List of utils to be reimplemented

## echo
- Parse flags (-n)
- Print operands to stdout


# Learnings

## How to have multiple binaries in a rust project
Just creating a `new.rs` file inside `bin/` directory makes cargo treat the project
as having multiple binaries. Then if you remove `main.rs`, `new.rs` will be the only
binary present.


Notes:
Interestingly, clippy does not seem to care whether you add a semicolon after a `println!`
statement or not.