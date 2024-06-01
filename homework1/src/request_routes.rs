use std::sync::Arc;

use axum::{
    routing::{delete, get},
    Router,
};
use sqlx::PgPool;

use crate::request_handlers::{
    delete_question, fetch_all_questions, get_question_by_id,
};

/// Sets up the routes for the application
///
/// # Arguments
///
/// * `database_pool` - A PostgresSQL connection pool wrapped with Arc
///
/// # Returns
///
/// Axum {Router} configured with the routes mentioned
pub fn setup_routes(database_pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/getAllQuestions", get(fetch_all_questions)) // Route to fetch all questions
        .route("/getQuestionByID/:id", get(get_question_by_id)) // Route to fetch a question by its ID
        .route("/deleteQuestion/:id", delete(delete_question))
        .with_state(database_pool)
    // .route("/addQuestion", post(add_question))
    // .route("/updateQuestion/:id", patch(update_question))
}
