use crate::questions_database::questions_module::QuestionStructure;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json};
use sqlx::PgPool;
use std::sync::Arc;
// use crate::questions_database::questions_module;

/// Retrieves every question from the database.
///
/// # Arguments
/// * `database_pool` - A state where the database pool is located
///
/// # Returns
/// A list of every query in the database contained in a JSON answer
pub async fn fetch_all_questions(State(database_pool): State<Arc<PgPool>>) -> impl IntoResponse {
    let all_questions = sqlx::query_as::<_, QuestionStructure>("SELECT * FROM questions_table")
        .fetch_all(&*database_pool)
        .await
        .expect("Error retrieving questions from the database");

    Json(all_questions)
}

/// Retrieves a question from the database using its ID.
///
/// # Arguments
/// * `q_id` - The question's ID to retrieve
/// * `database_pool` - A state where the pool of database connections is located
///
/// # Returns
/// If the query is found, a JSON answer with it, or if it is not, an error message
pub async fn get_question_by_id(
    Path(q_id): Path<i32>,
    State(database_pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    let query_result = sqlx::query_as::<_, QuestionStructure>(
        "SELECT * FROM questions_table WHERE question_id = $1",
    )
    .bind(q_id)
    .fetch_optional(&*database_pool)
    .await;

    if let Ok(Some(question)) = query_result {
        (StatusCode::OK, Json(question)).into_response()
    } else if let Ok(None) = query_result {
        let error_message =
            json!({"error":"Question with this specific if not found or it doesn't exists!"});
        (StatusCode::NOT_FOUND, Json(error_message)).into_response()
    } else {
        let error_message = json!({"error": "Internal server error"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
    }
}

/// Uses the question's ID to remove it from the database.
///
/// # Arguments
/// * `q_id` - The question's ID to delete
/// * `database_pool` - A state that contains the database connection pool
///
/// # Returns
/// When a question is removed, a success message appears; if the question cannot be located, an error message appears.
pub async fn delete_question(
    Path(q_id): Path<i32>,
    State(database_pool): State<Arc<PgPool>>,
) -> impl IntoResponse {
    let delete_result = sqlx::query("DELETE FROM questions_table WHERE question_id = $1")
        .bind(q_id)
        .execute(&*database_pool)
        .await;

    if let Ok(question_deleted) = delete_result {
        if question_deleted.rows_affected() > 0 {
            let success_message = json!({"message":"Question deleted successfully"});
            (StatusCode::OK, Json(success_message)).into_response()
        } else {
            let error_message =
                json!({"error":"Question with this specific if not found or it doesn't exists!"});
            (StatusCode::NOT_FOUND, Json(error_message)).into_response()
        }
    } else {
        let error_message = json!({"error": "Internal server error"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
    }
}

// /// Adds a new question to the questions' database.
// /// This function takes a JSON payloads as input and attempts to add a new entry to the database
// pub async fn add_question(Json(input): Json<Value>) -> impl IntoResponse {
//     let mut database = crate::questions_database::QUESTIONS_DATABASE
//         .write()
//         .unwrap();
//     // Check if the input payload contains the required fields
//     let required_fields = [
//         "question_id",
//         "question_title",
//         "type_of_content",
//         "type_of_question",
//     ];
//     let missing_fields = required_fields
//         .iter()
//         .find(|&&field| input.get(field).is_none());
//
//     if let Some(field) = missing_fields {
//         let error_response = serde_json::json!({
//             "error": format!("Missing fields. Required: {}", field)
//         });
//         return Err((StatusCode::BAD_REQUEST, Json(error_response)));
//     }
//
//     let question = extracted_question(&input);
//
//     // Check if a question with the specified ID already exists
//     if database
//         .iter()
//         .any(|q| q.question_id == question.question_id)
//     {
//         let error_response = serde_json::json!({
//             "error": "Duplicate ID exists"
//         });
//         return Err((StatusCode::CONFLICT, Json(error_response)));
//     }
//
//     database.push(question);
//     let response_body = serde_json::json!({
//         "message": "Question added successfully"
//     });
//     Ok(Json(response_body))
// }
//
// /// Extracts a question object from a JSON payload as a input
// fn extracted_question(input: &Value) -> questions_module::Question {
//     questions_module::Question {
//         // Extract the required fields from the input payload
//         question_id: input["question_id"].as_str().unwrap().to_string(),
//         question_title: input["question_title"].as_str().unwrap().to_string(),
//         type_of_content: input["type_of_content"].as_str().unwrap().to_string(),
//         type_of_question: input["type_of_question"]
//             .as_array()
//             .unwrap()
//             .iter()
//             .map(|type_of_question| type_of_question.as_str().unwrap().to_string())
//             .collect(),
//     }
// }
//
// /// This function retrieves a question by its `id` and updates its fields. If the specified question exists,
// /// its details are updated or an error response if not found.
// pub async fn update_question(
//     Path(id): Path<String>,
//     Json(input): Json<Value>,
// ) -> impl IntoResponse {
//     let mut database = crate::questions_database::QUESTIONS_DATABASE
//         .write()
//         .unwrap();
//
//     if let Some(question_index) = database.iter().position(|q| q.question_id == id) {
//         let mut current_question = database[question_index].clone();
//
//         if let Some(question_title) = input.get("question_title") {
//             current_question.question_title = question_title.as_str().unwrap().to_string();
//         }
//         if let Some(type_of_content) = input.get("type_of_content") {
//             current_question.type_of_content = type_of_content.as_str().unwrap().to_string();
//         }
//         if let Some(type_of_question) = input.get("type_of_question") {
//             current_question.type_of_question = type_of_question
//                 .as_array()
//                 .unwrap()
//                 .iter()
//                 .map(|type_of_question| type_of_question.as_str().unwrap().to_string())
//                 .collect();
//         }
//
//         database[question_index] = current_question.clone();
//
//         let response_body = serde_json::json!({
//             "message": "Question updated the successfully of current question index",
//         });
//         Ok(Json(response_body))
//     } else {
//         let error_response = Json(serde_json::json!({
//             "error": "Question does not exist!"
//         }));
//         Err((StatusCode::NOT_FOUND, error_response))
//     }
// }