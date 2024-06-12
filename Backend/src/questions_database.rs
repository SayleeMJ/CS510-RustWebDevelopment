use sqlx::{PgPool, Pool, Postgres};
use std::env;

/// Defines the structure and functionalities for managing questions
pub mod questions_module {

    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    /// Represents a single question in the database
    /// Each question includes an ID, title, content type and a list of question categories
    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]    pub struct QuestionStructure {
        pub question_id: i32,
        pub question_title: String,
        pub type_of_content: String,
        pub type_of_question: Vec<String>,
    }
}

/// Creates the PostgresSQL database connection pool from scratch.
pub async fn initialize_questions_database() -> Pool<Postgres> {
    let question_database_url =
        env::var("QUESTION_DATABASE_URL").expect("QUESTION_DATABASE_URL must be set in .env file");
    PgPool::connect(&question_database_url)
        .await
        .expect("Couldn't connect to the database. Please check connection or database url")
}
