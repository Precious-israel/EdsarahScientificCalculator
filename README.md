# Overview

I am learning Rust to develop a strong foundation in systems programming and 
memory-safe software design. As a software engineer, my goal is to build 
reliable applications that perform well and fail predictably. Rust's ownership 
model, its compile-time safety guarantees, and its explicit error handling make 
it an ideal language for that goal, so I chose to spend this module deepening 
my understanding of these features.

For this module I wrote a **command-line scientific calculator** for Edsarah 
Software and Solution. The program evaluates a single calculation passed as 
command-line arguments, or it starts an interactive prompt when no arguments 
are supplied. It supports basic arithmetic, powers, the recursive factorial 
function, square roots, exponential and logarithmic functions, and 
trigonometric functions. Every fallible operation returns a descriptive error 
instead of panicking, and results that would overflow are detected and reported 
rather than silently printed as incorrect numbers.

My purpose in writing this software was to practice the core Rust concepts 
required by this module: variables (both mutable and immutable), expressions, 
conditionals, loops, functions with ownership and references, and a data 
structure (`Vec`). I also wanted to demonstrate proper error handling using 
`Result`, the `?` operator, and a custom error enum, along with compile-time 
overflow protection using `checked_mul`. I wrote 35 unit tests to confirm the 
correctness of the recursion, the parsers, the dispatcher, and the formatter.

[Software Demo Video](https://www.loom.com/share/170335e6f1e14299925576fff8e3ea90)

# Development Environment

I developed the software on Windows using **Visual Studio Code** as my editor, 
with the **rust-analyzer** extension for code completion and inline diagnostics 
and **CodeLLDB** for step debugging. The Rust toolchain was installed through 
**rustup**, and **Cargo** was used to build, test, run, and format the project. 
Version control was handled with **Git** and hosted on **GitHub** at 
[EdsarahScientificCalculator](https://github.com/Precious-israel/EdsarahScientificCalculator).

The software is written in **Rust (edition 2021)** and depends only on the 
standard library — no external crates are used. The unit tests use Rust's 
built-in `#[cfg(test)]` module and `assert_eq!` macro.

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings — Small Interactive Exercises](https://github.com/rust-lang/rustlings)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)

# Future Work

- Add support for user-defined variables and full expression evaluation, so 
  that input like `x = 5; x * 2` becomes possible.
- Add a history feature to the interactive prompt so the user can recall past 
  calculations with the up-arrow key.
- Extend the factorial function to arbitrary-size results using a big-integer 
  crate, so values beyond `20!` can be computed.
- Add a `--precision` flag to control how many decimal places are printed in 
  the output.
- Support reading a list of calculations from a file so batch jobs can be run 
  in a single command.