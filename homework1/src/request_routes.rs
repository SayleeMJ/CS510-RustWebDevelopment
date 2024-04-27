use axum::{routing::get, Router};

use crate::{
    questions_database,
    request_handlers::{get_question_by_id, get_questions},
};

pub fn setup_routes() -> Router {
    questions_database::initialize_questions_database();

    Router::new()
        .route("/getQuestions", get(get_questions))
        .route("/getQuestionByID/:id", get(get_question_by_id))
}
