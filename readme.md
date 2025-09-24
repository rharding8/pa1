# Parallel Prime Finder
## Ryan Harding, Due 9/24, COP6616

# Compilation and Execution
## RECOMMENDED: Cargo
As I used Cargo as my environment for this project, executing with Cargo is recommended. To do so, run:
    cargo run
    OR
    cargo run --release
From the root of the project. The latter simply builds the project in target/release instead of target/debug, but both work.
To compile without running, run:
    cargo build
    OR
    cargo build --release

## Without Cargo
As the program uses all standard Rust libraries, it will compile and run without cargo, just using the "main.rs" file. To do so, run:
    rustc main.rs
    ./main
The first command compiles, the second executes. Doing so outputs the results in the directory with "main.rs", not the project root.

# Other Files
"sequential.txt" is an example output produced using a sequential sieve.
"pa1description.pdf" is the original assignment pdf from Webcourses.
"pa1analysis.pdf" is the analysis document for the assignment.