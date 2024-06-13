# Rust Web Example
***Name: Saylee Jagtap***

This repository is for homework (course CS510-RustWebDevelopment Spring 2024)


## Assignment (Rust REST Front End)

This is a web application created as part of the CS510-RustWebDevelopment course in Spring 2024. It uses the Rust programming language to provide a RESTful API for managing a question database, with the Axum framework for the backend and the Warp framework for the frontend.

The backend uses a PostgresSQL database to store, retrieve, update, and delete questions. It demonstrates the efficient handling of HTTP requests, data validation, and error management for online services.

The front end, developed with Warp, provides a user-friendly interface enabling seamless API interaction. This covers viewing all questions, retrieving information about specific questions, adding new questions, updating existing ones, and deleting questions.


### Backend Project Structure
- src/main.rs - The main entry point for the application and handles the server functionality.
- src/questions_database.rs - Module for managing the question database.
- src/request_handlers.rs - Contains handlers for API routes.
- src/request_routes.rs - Sets up the route configuration.

### Frontend Project Structure
- `main.rs`: Sets up the Warp server and configures the routing.
- `route_handlers.rs`: Defines the logic for processing API requests to the backend.
- `routers.rs`: Sets up the routes for providing static files and managing API queries.
- `templates/`: Contains static files for the front-end user interface.
    - `index.html`: Main HTML file used to structure the web page.
    - `index.css`: A stylesheet defines the visual appearance of a web page.
    - `index.js`: The JavaScript file is used to communicate with the backend API and dynamically update the web page.


### Backend Features
- GET /getQuestions - Retrieve a list of all questions.
- GET /getQuestionByID/:id - Retrieve a specific question by its ID.
- DELETE /deleteQuestion/:id - Delete a question by its ID.
- CREATE /addQuestion - Add a new question.
- UPDATE /updateQuestion/:id - Updates a question of specific ID.

### Prerequisites
- Rust and Cargo installed (https://www.rust-lang.org/tools/install)
- PostgresSQL installed (https://www.postgresql.org/download/)

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

#### Backend Installation:

```bash
git clone https://github.com/SayleeMJ/CS510-RustWebDevelopment.git

cd CS510-RustWebDevelopment/Backend
```

- Start the server:
```bash
cargo run
```
*The server will typically listen on port http://localhost:1000/*

#### Frontend Installation

```bash
cd CS510-RustWebDevelopment/Frontend
```

- Start the server:
```bash
cargo run
```
*The server will typically listen on port http://localhost:2000/*

### References
- https://github.com/pdx-cs-rust-web/knock-knock/tree/main
- https://docs.rs/axum/latest/axum/
- https://github.com/Rust-Web-Development/code
- https://crates.io/crates/deadpool-postgres
- https://www.youtube.com/watch?v=n1B3B_nodR8&ab_channel=TPPZbuildsthings
- https://www.codingame.com/playgrounds/365/getting-started-with-rust/primitive-data-types#:~:text=integer%20types,an%20unsigned%2C%2064%20bit%20integer.
