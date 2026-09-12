# Edsarah Scientific Calculator

A command line scientific calculator written in Rust for **Edsarah Software and Solution**.

The program evaluates a single calculation passed as command line arguments, or
starts an interactive prompt when no arguments are supplied. All calculation
logic lives in a small library so that it can be unit tested independently of
the user interface.

---

## 1. Author


| Name | ISRAEL OJO |
| Course CSE 310| RUST |
| Organisation | Edsarah Software and Solution |
| Repository | https://github.com/Precious-israel/EdsarahScientificCalculator |

---

## 2. Video Demonstration

**Video link:** https://www.loom.com/share/170335e6f1e14299925576fff8e3ea90

> The video is 4–5 minutes long and includes a talking-head image of the author
> presenting the project, as required for accreditation. It walks through:
> 1. Running the calculator from the command line and from the interactive prompt.
> 2. A code walkthrough of `src/lib.rs` (recursion, error handling, overflow checks).
> 3. A code walkthrough of `src/main.rs` (argument parsing and the interactive loop).
> 4. Running the unit test suite with `cargo test`.

---

## 3. Description

The Edsarah Scientific Calculator supports basic arithmetic, powers, the
recursive factorial function, roots, logarithms and trigonometric functions.
Every fallible operation returns a descriptive error rather than panicking, and
results that would overflow are detected and reported instead of being printed
as a silently incorrect value.

### Features

* **Two input modes** — command line arguments or an interactive `calc>` prompt.
* **Recursive factorial** — `factorial(n)` calls itself until it reaches `0! = 1`.
* **Recursive exponentiation** — fast exponentiation via `power_positive`.
* **Overflow protection** — `checked_mul` guards the factorial, and every result
  is checked for infinity before it is displayed.
* **Domain validation** — `sqrt(-1)` and `ln(0)` produce clear domain errors.
* **Helpful error messages** — unknown operations, wrong operand counts,
  negative factorial input and unparsable numbers are all reported clearly.
* **Fully unit tested** — 35 unit tests covering the recursion, the parser,
  the dispatcher and the formatter.

### Supported operations

| Operation | Aliases | Operands | Description |
| --- | --- | --- | --- |
| `add` | `+`, `plus` | `a b` | `a + b` |
| `sub` | `-`, `minus`, `subtract` | `a b` | `a - b` |
| `mul` | `*`, `x`, `multiply` | `a b` | `a * b` |
| `div` | `/`, `divide` | `a b` | `a / b` |
| `pow` | `^`, `power` | `a b` | `a` raised to the power `b` |
| `factorial` | `fact`, `!` | `n` | `n!`, recursive, `0 <= n <= 20` |
| `sqrt` | `root` | `a` | Square root of `a` |
| `exp` | | `a` | `e` raised to the power `a` |
| `ln` | | `a` | Natural logarithm |
| `log10` | `log` | `a` | Base-10 logarithm |
| `sin`, `cos`, `tan` | | `a` | Trigonometric functions (radians) |
| `abs` | | `a` | Absolute value |

---

## 4. Requirements

* **Rust** 1.61 or newer (the project uses `std::process::ExitCode`).
  Install from <https://rustup.rs>.
* **Cargo** (installed together with Rust).
* Any operating system: Windows, macOS or Linux.

---

## 5. Getting Started

```bash
# 1. Clone the repository
git clone https://github.com/Precious-israel/EdsarahScientificCalculator.git
cd EdsarahScientificCalculator

# 2. Build the project
cargo build --release

# 3. Run the tests
cargo test

# 4. Run the program
cargo run --release -- factorial 5