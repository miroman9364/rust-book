# Rust Book Code Along

This project represents my walkthrough of the book [The Rust Programming Language](https://rust-book.cs.brown.edu/ch01-01-installation.html) ([Brown edition](https://rust-book.cs.brown.edu/experiment-intro.html)/[stable](https://doc.rust-lang.org/book/ch01-01-installation.html)).

## Projects

Here's a list of the projects I created while learning from the Rust book, along with the commands to run them:

- [`hello_world`](./projects/hello_world/main.rs) is the most basic Rust app. You need to build it and then execute it:

  ```bash
  # compile the code
  rustc --out-dir projects/hello_world projects/hello_world/main.rs

  # run the app
  projects/hello_world/main
  ```

- [`hello_cargo`](./projects/hello_cargo/src/main.rs) is the ever so slightly more advanced application made using the Rust package manager, `cargo`.

  ```bash
  # build the app
  cargo build --manifest-path projects/hello_cargo/Cargo.toml

  # run the app (you can just call run and `cargo` will build if necessary)
  cargo run --manifest-path projects/hello_cargo/Cargo.toml
  ```

- [`guessing_game`](./projects/guessing_game/src/main.rs) is my implementation of the guessing game. I separated out the code into helper functions, and added a few embellishments to make it more interesting and try some things that aren't covered in this early part of the book.

  ```bash
  # build and run the app
  cargo run --manifest-path projects/guessing_game/Cargo.toml
  ```

- [`scratch`](./projects/scratch/src/main.rs) this is where I put a lot of the little exercises from chapter 3. Each exercise has its own function, and they all get called from `main`.

  ```bash
  cargo run --manifest-path projects/scratch/Cargo.toml
  ```

## Chapter 1

In [chapter 1](https://rust-book.cs.brown.edu/ch01-00-getting-started.html) you learn the following basics to install, write, compile and execute a Rust binary app:

- Installing Rust on [Linux](https://rust-book.cs.brown.edu/ch01-01-installation.html#installing-rustup-on-linux-or-macos), [macOS](https://rust-book.cs.brown.edu/ch01-01-installation.html#installing-rustup-on-linux-or-macos), or [Windows](https://rust-book.cs.brown.edu/ch01-01-installation.html#installing-rustup-on-windows)
- Open a local copy of the Rust book: `rustup doc`
- Create a simple "Hello, world" app and compile with basic toolchain
- Create a crate for a simple "Hello, world" app, build and run with the cargo tool

## Chapter 3

The book allows for you to do chapters 2 and 3 out-of-order, so that's what I did. In [chapter 3](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html) you learn a lot of the basic things that Rust has in common with most programming languages:

- you learn about variables
- you learn the primary data types
- you learn about functions and return values
- you learn the differences between statements and expressions
- you learn about comments
- you learn about conditional expressions
- you learn about control expressions like `if`/`else`
- you learn about loop expressions like `loop`, `while` and `for`
- you learn how to turn expressions into statements

## Chapter 2

In [chapter 2](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html) you build a guessing game. The book allows for you to complete chapter 3 first, which is the approach I took.

In this chapter, you create a simple game to guess at a random number. While writing this program, you use the following:

- you add a dependency to your binary create
- you use the `std::io` to read a line in from the console
- you parse a string into a number
- you use a loop construct
- you use a match expression

In addition to what the book covers, I embellished as follows:

- I tried to make things a little more modular by breaking different steps into helper functions
- I created multiple exit conditions
- I gave the ability to ask for a hint/help
- I made it so typing a non-number guess doesn't cause a panic

## Chapter 4

In [chapter 4](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html) you learn about ownership. This seems to be the biggest thing that sets Rust apart from most other languages. This is the thing I know nothing about. ☺️

A lot of the following is quoted and/or summarized directly from the Rust book:

- "Safety is the absence of undefined behavior." That makes sense.
- Rust avoids _runtime-checks_ by doing extensive _compile-time_ checks.
- "A foundational goal of Rust is to ensure that your programs never have undefined behavior."
- A secondary goal of Rust is to prevent undefined behavior at compile-time instead of run-time.
  - Catching bugs at compile-time means avoiding those bugs in production, improving the reliability of your software.
  - Catching bugs at compile-time means fewer runtime checks for those bugs, improving the performance of your software.
- Rust cannot prevent all bugs. You can still have bad-logic that is programmatically safe, but causes unexpected and undesirable results, such as an unauthenticated API to what is intended to be secure data.
- The Rust Reference maintains a large list of ["Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).

### TODO: Aquascope

Look into how you might temporarily install [Aquascope](https://github.com/cognitive-engineering-lab/aquascope) into the devcontainer and try playing with it to see what's happening under the covers.

[//]: # (cSpell:ignore Aquascope,)
