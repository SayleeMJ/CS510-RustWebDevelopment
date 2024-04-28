use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::Value;
use crate::questions_database::questions_module;

/// Returns all questions from the database
pub async fn get_questions() -> impl IntoResponse {
    let questions_db = crate::questions_database::QUESTIONS_DATABASE
        .read()
        .unwrap();
    Json(questions_db.clone())
}


/// Gets a specific question from the database by its unique ID
/// Returns the question if found, or an error response if not found
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

/// Deletes a specific question from the database by its unique ID
/// Returns a success response if found and the question was deleted, or an error response if not found
pub async fn delete_question(Path(id): Path<String>) -> impl IntoResponse {
    let mut database = crate::questions_database::QUESTIONS_DATABASE
        .write()
        .unwrap();
    if let Some(question_index) = database.iter().position(|q| q.question_id == id) {
        database.remove(question_index);
        let response_body = serde_json::json!({"status":"Deleted Successfully!"});

        Ok(Json(response_body))
    } else {
        let error_response = serde_json::json!({"error":"Question does not exist!"});

        Err(Json(error_response))
    }
}

/// Adds a new question to the questions' database.
/// This function takes a JSON payloads as input and attempts to add a new entry to the database
pub async fn add_question(Json(input): Json<Value>) -> impl IntoResponse {
    let mut database = crate::questions_database::QUESTIONS_DATABASE.write().unwrap();
    // Check if the input payload contains the required fields
    let required_fields = ["question_id", "question_title", "type_of_content", "type_of_question"];
    let missing_fields = required_fields.iter().find(|&&field| input.get(field).is_none());

    if let Some(field) = missing_fields {
        let error_response = serde_json::json!({
            "error": format!("Missing fields. Required: {}", field)
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let question = extracted_question(&input);

    // Check if a question with the specified ID already exists
    if database.iter().any(|q| q.question_id == question.question_id) {
        let error_response = serde_json::json!({
            "error": "Duplicate ID exists"
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    database.push(question);
    let response_body = serde_json::json!({
        "message": "Question added successfully"
    });
    Ok(Json(response_body))
}

/// Extracts a question object from a JSON payload as a input
fn extracted_question(input: &Value) -> questions_module::Question {
    questions_module::Question {
        // Extract the required fields from the input payload
        question_id: input["question_id"].as_str().unwrap().to_string(),
        question_title: input["question_title"].as_str().unwrap().to_string(),
        type_of_content: input["type_of_content"].as_str().unwrap().to_string(),
        type_of_question: input["type_of_question"]
            .as_array()
            .unwrap()
            .iter()
            .map(|type_of_question| type_of_question.as_str().unwrap().to_string())
            .collect(),
    }
}

/// This function retrieves a question by its `id` and updates its fields. If the specified question exists,
/// its details are updated or an error response if not found.
pub async fn update_question(Path(id): Path<String>, Json(input): Json<Value>) -> impl IntoResponse {
    let mut database = crate::questions_database::QUESTIONS_DATABASE
        .write()
        .unwrap();

    if let Some(question_index) = database.iter().position(|q| q.question_id == id) {
        let mut current_question = database[question_index].clone();

        if let Some(question_title) = input.get("question_title") {
            current_question.question_title = question_title.as_str().unwrap().to_string();
        }
        if let Some(type_of_content) = input.get("type_of_content") {
            current_question.type_of_content = type_of_content.as_str().unwrap().to_string();
        }
        if let Some(type_of_question) = input.get("type_of_question") {
            current_question.type_of_question = type_of_question
                .as_array()
                .unwrap()
                .iter()
                .map(|type_of_question| type_of_question.as_str().unwrap().to_string())
                .collect();
        }

        database[question_index] = current_question.clone();

        let response_body = serde_json::json!({
            "message": "Question updated the successfully of current question index",
        });
        Ok(Json(response_body))
    } else {
        let error_response = Json(serde_json::json!({
            "error": "Question does not exist!"
        }));
        Err((StatusCode::NOT_FOUND, error_response))
    }
}