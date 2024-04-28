# Rust Web Example
***Name: Saylee Jagtap*** 

This repository is for homework (course CS510-RustWebDevelopment Spring 2024)


## Homework 1 (Get The REST Up)

This project in Rust includes a RESTful API for question database management constructed with the web framework Axum.

### Project Structure
- src/main.rs - The main entry point for the application and handles the server functionality.
- src/questions_database.rs - Module for managing the question database.
- src/request_handlers.rs - Contains handlers for API routes.
- src/request_routes.rs - Sets up the route configuration.

### Features
- GET /getQuestions - Retrieve a list of all questions.
- GET /getQuestionByID/:id - Retrieve a specific question by its ID.
- DELETE /deleteQuestion/:id - Delete a question by its ID.
- CREATE /addQuestion - Add a new question.
- UPDATE /updateQuestion/:id - Updates a question of specific ID.

### Prerequisites
Rust and Cargo installed (https://www.rust-lang.org/tools/install)

### Installation

-  Clone this repository and go to the specific folder:

```bash
git clone https://github.com/SayleeMJ/CS510-RustWebDevelopment.git

cd CS510-RustWebDevelopment/homework1
```

- Start the server:
```bash
cargo run
```
*The server will typically listen on port 1000.*

### References
- https://github.com/pdx-cs-rust-web/knock-knock/tree/main
- https://docs.rs/axum/latest/axum/
- https://github.com/Rust-Web-Development/code
