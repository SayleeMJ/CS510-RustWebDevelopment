# Rust Web Example
***Name: Saylee Jagtap***

This repository is for homework (course CS510-RustWebDevelopment Spring 2024)


## Assignment (Persistent Data)

This project in Rust includes a RESTful API for question database management constructed with the web framework Axum and Persistent database such as PostgresSQL.

### Project Structure
- src/main.rs - The main entry point for the application and handles the server functionality.
- src/questions_database.rs - Module for managing the question database.
- src/request_handlers.rs - Contains handlers for API routes.
- src/request_routes.rs - Sets up the route configuration.

### Features
- GET /getAllQuestions - Retrieve a list of all questions.
- GET /getQuestionByID/:id - Retrieve a specific question by its ID.
- DELETE /deleteQuestion/:id - Delete a question by its ID.
- CREATE /addQuestion - Add a new question.
- UPDATE /updateQuestion/:id - Updates a question of specific ID.

### Prerequisites
- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- PostgreSQL installed (https://www.postgresql.org/download/)

### Database Setup
Create the database:
```bash
CREATE DATABASE questions;
```

Create the Table:
```bash
CREATE TABLE IF NOT EXISTS questions_table (
    question_id SERIAL PRIMARY KEY,
    question_title VARCHAR NOT NULL,
    type_of_content TEXT NOT NULL,
    type_of_question VARCHAR[] NOT NULL
);
```

Inserting data into the database:

```bash
INSERT INTO questions_table (question_title, type_of_content, type_of_question)
VALUES ('Hello', 'How are you?', '{"Greeting"}');
```


### Installation

-  Clone this repository and go to the specific folder:

```bash
git clone https://github.com/SayleeMJ/CS510-RustWebDevelopment.git

cd CS510-RustWebDevelopment/Assignment
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
- https://crates.io/crates/deadpool-postgres
- https://www.youtube.com/watch?v=n1B3B_nodR8&ab_channel=TPPZbuildsthings
- https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types#:~:text=integer%20types,an%20unsigned%2C%2064%20bit%20integer.
