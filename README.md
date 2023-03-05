# Todolist App
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple todo list CLI app that uses a postgres database to store tasks.

## Setting Up
1. Fill in the placeholders in [.env.example](.env.example) and rename the file to just `.env`

2. Ensure that you have postgres running on your machine with a database named `todolist_db`

3. Install [diesel_cli](https://crates.io/crates/diesel_cli) with only the `postgres` feature:
```bash
cargo install diesel_cli --no-default-features --features postgres
```

4. Prepare the `todolist_db` schemas:
```bash
diesel migration run
```

5. Build the package and run it!
```bash
cargo build -r
# or
cargo build --release
```
```bash
./target/release/todo
# if you're on windows, use `todo.exe`
```