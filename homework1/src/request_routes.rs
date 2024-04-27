use axum::{routing::get, Router};
use axum::routing::{delete, post};

use crate::{
    questions_database,
    request_handlers::{get_question_by_id, get_questions,delete_question, add_question},
};

/// Sets up the routes for the application
pub fn setup_routes() -> Router {
    questions_database::initialize_questions_database();

    Router::new()
        .route("/getQuestions", get(get_questions))
        .route("/getQuestionByID/:id", get(get_question_by_id))
        .route("/deleteQuestion/:id", delete(delete_question))
        .route("/addQuestion", post(add_question))

}
