[![Clippy](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml)


# Package a Python Script into a Command-Line Tool

## Purpose:
This template offering a unique approach to create functions, using Rust and package a Rust Script into a Command-line tool. I selected the folder `cli-salad`, then read and run the code in `lib.rs` and `main.rs`, to introduce the `create_fruit_salad` function. This function curates a selection of Portuguese fruits, presenting a randomized mix upon request.  The addition of test.rs ensures the tool's reliability, with all functions being rigorously tested. Through GitHub Actions and the provided Makefile, and got 100% pass.

## This repo contains:
Certainly! Here's a brief description for each of the files in the `cli-salad` repo:

- **Makefile**: A script containing a set of tasks used for compiling and building the project. It simplifies and standardizes the build process, ensuring consistency in project execution.

- **lib.rs**: The library file that houses the primary logic of the tool. This is where the `create_fruit_salad` function resides, which forms the essence of the `cli-salad` application. The function picks a random assortment of fruits from a predefined list.

- **main.rs**: The entry point of the `cli-salad` tool. It integrates with the command-line interface, parsing user inputs and calling appropriate functions from `lib.rs`. It facilitates user interaction and displays the resulting fruit salad.

- **test.rs**: Contains unit tests that validate the functionality and reliability of the tool's features. These tests ensure that both the provided and newly-added functions perform as expected under various conditions.

- **Cargo.toml**: The manifest file for Rust projects. It provides metadata and declares package dependencies, ensuring that the project has all the necessary components for building and running successfully.

## Steps:

1. Fork the repository at **https://github.com/nogibjj/rust-data-engineering**

2. Clone your forked repository 

3. Navigate to one of the command-line tool projects

4. Make a small modification to the tool such as:

   - Adding a new command line argument
    
   - Supporting additional input file formats
    
   - Adding more processing logic
    
   - Changing output formatting

5. Run `cargo build` to compile your changes  

6. Run `cargo run` to test your modified tool

7. Commit your changes and push to your forked repository

## User Guide for 'cli-salad':
1. Ensure you have Rust and Cargo installed on your machine
2. Check if cargo.toml is under the project folder
3. Navigate to the directory containing the `cli-salad` source code using a terminal or command prompt
   `cd cli-salad`
4. Generate a random fruit salad with a specified number of fruits:
   `cli-salad -- --number <number of fruits>` (replace <number of fruits> with the number of fruits you want in your salad
   e.g: to generate a fruit salad with 5 fruits:
   <img width="682" alt="Screen Shot 2023-10-25 at 23 39 59" src="https://github.com/nogibjj/IDS706_miniproject7_xk10/assets/143849077/3fe42cfc-206a-4370-b65f-8655d651cc85">
This command will display a random selection of 5 fruits that can be part of your salad.
5. test the tool:
   To run tests to ensure the functionality of the tool: `cargo test`
   By using `cargo build`, users can compile the code, making it ready for execution. Once built, `cargo run` can be employed to initiate the cli-salad tool, generating random fruit salads based on user input.
<img width="599" alt="Screen Shot 2023-10-25 at 23 43 34" src="https://github.com/nogibjj/IDS706_miniproject7_xk10/assets/143849077/824c75ce-3656-42e4-bc6a-5c9a4f178da1">

## Makefile

Each subdirectory project uses this style to make it easy to test and run

```
format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

all: format lint test run
```


## References

* [Rust Collections](https://doc.rust-lang.org/std/collections/index.html)
* [GitHub Copilot CLI](https://www.npmjs.com/package/@githubnext/github-copilot-cli)
* [Rust Fundamentals](https://github.com/alfredodeza/rust-fundamentals)
* [Rust Tutorial](https://nogibjj.github.io/rust-tutorial/)
* [Rust MLOps Template](https://github.com/nogibjj/mlops-template)
