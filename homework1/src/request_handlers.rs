use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn get_questions() -> impl IntoResponse {
    let questions_db = crate::questions_database::QUESTIONS_DATABASE
        .read()
        .unwrap();
    Json(questions_db.clone())
}

pub async fn get_question_by_id(Path(id): Path<String>) -> impl IntoResponse {
    let database = crate::questions_database::QUESTIONS_DATABASE
        .read()
        .unwrap();

    if let Some(question_detail) = database.iter().find(|q| q.question_id == id).cloned() {
        Ok(Json(question_detail))
    } else {
        let error_response = Json(serde_json::json!({
            "error": "Question ID not found. Ensure it's correct or add the question."
        }));
        Err((StatusCode::NOT_FOUND, error_response))
    }
}
