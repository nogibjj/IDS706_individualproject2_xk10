

# Individual Project#2: Rust CLI Binary with SQLite

## Purpose:
This repository hosts a Rust CLI tool that manages a SQLite database for the purpose of handling a Drinks dataset. The tool is designed to perform a sequence of operations that include extracting, transforming, loading, and querying data. It starts by downloading a dataset from a specified GitHub URL and saving it as a CSV file within the local repository. Following this, the data from the CSV file is transformed to fit the SQLite database schema and then loaded into the database. Users can then execute SQL queries through the CLI tool to interact with the database, allowing for retrieval and manipulation of the Drinks data in a structured and efficient manner.

## Github Actions Workflows:
[![Rust CI/CD](https://github.com/nogibjj/IDS706_individualproject2_xk10/actions/workflows/CICD.yml/badge.svg)](https://github.com/nogibjj/IDS706_individualproject2_xk10/actions/workflows/CICD.yml)
This repository has CI/CD set up in Github Actions Workflows. The workflows include:
1. building, formatting, linting, testing, generate and push
2. specific query commands: create, delete, read, update
3. Archive Binary: Binary is also incorporate into the Github Actions. For downloading the binary, go to Github Actions, click on one of the Workflows, go to the bottom of the page and download the file.
   
## Steps:

1. Fork the repository from miniproject7

5. Run `cargo build` to compile your changes  

6. Run `cargo run` to test your modified tool


## Check format and test:
In order to check format and test,
run `make format`, `make lint`, and `cargo test`
<img width="573" alt="Screen Shot 2023-11-04 at 15 11 22" src="https://github.com/nogibjj/IDS706_individualproject2_xk10/assets/143849077/b7cffd92-f90f-4d8b-849d-025bad52cb16">
<img width="646" alt="Screen Shot 2023-11-04 at 16 58 40" src="https://github.com/nogibjj/IDS706_individualproject2_xk10/assets/143849077/b0682650-f014-42d3-b4e6-a82ebacb19b4">

## Github Copilot
GitHub Copilot proved to be an invaluable asset specifically tailored to Rust development in our CLI project. Rust's stringent compile-time guarantees and ownership model present a unique set of challenges and learning opportunities. Here’s how Copilot was particularly beneficial:
- **Understanding Rust Semantics**: GitHub Copilot suggested code that adhered to Rust’s strict ownership and borrowing rules, speeding up development by reducing the trial-and-error typically associated with learning and using Rust's memory safety guarantees.
- **Leveraging Rust's Type System**: It provided suggestions that made full use of Rust's expressive type system, helping us to define more precise and robust data structures and function signatures.
- **Concurrent Programming Guidance**: Rust’s fearless concurrency is one of its standout features. Copilot helped navigate Rust's concurrency primitives like `Arc`, `Mutex`, and `Channels`, to write safe concurrent code with less effort.
- **Macro Use and Expansion**: Copilot offered guidance on using Rust's powerful macro system for code reuse and metaprogramming, which can be daunting due to its complexity.
- **Dependency Management**: It aided in managing dependencies through Cargo.toml suggestions, keeping our project up-to-date with the Rust ecosystem.
- **Idiomatic Expressions**: The AI assistant promoted Rust idioms and best practices, ensuring that our code was not only functional but also clean and idiomatic, aligning with the Rust community’s standards.




