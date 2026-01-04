/*
echo

Implementation Plan:

1. Get the arguments passed to the cli (equivalent of argc and *argv)
2. Print everything passed to the cli to stdout
3. Implement check for -n flag
*/

fn main() {
    // Simple clone of `echo`

    let mut args = std::env::args().skip(1).peekable();

    let newline = args.peek().map(|s| s.as_str()) != Some("-n");

    if !newline {
        args.next(); // skip -n
    };

    if let Some(first_arg) = args.next() {
        print!("{first_arg}");

        for arg in args {
            print!(" {arg}");
        }
    }

    if newline {
        println!();
    }
}

/*
Notes:

1. Without handling `-n` flag
The program could be implemented more succintly like so:

fn main() {
    println!("{}", std::env::args().skip(1).collect::<Vec<_>>().join(" "));
}

However, this implementation calls `collect` and `join`, which leads to memory
allocation.

The current implementation does not lead to extra memory allocation


2. `-n` flag handling
The key observation is that the `-n` flag can only be passed as the first
argument. It does not have any effect if it is passed later. So we can simply do
some special handling for the first character.

Earlier I was doing a separate call for first element and print loop, but we
can make the observation that only if there is a first element will the loop
ever run. So we can put the print loop inside the if let for the first argument

*/
