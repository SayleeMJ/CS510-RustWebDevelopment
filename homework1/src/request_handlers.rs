use crate::questions_database::questions_module::QuestionStructure;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json, Value};
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
    // Attempt to fetch all questions from the database
    let all_questions = sqlx::query_as::<_, QuestionStructure>("SELECT * FROM questions_table")
        .fetch_all(&*database_pool)
        .await
        .expect("Error retrieving questions from the database");

    // Respond with the list of all questions in JSON format
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
    // Attempt to fetch the question from the database
    let query_result = sqlx::query_as::<_, QuestionStructure>(
        "SELECT * FROM questions_table WHERE question_id = $1",
    )
    .bind(q_id)
    .fetch_optional(&*database_pool)
    .await;

    if let Ok(Some(question)) = query_result {
        // If found, reply with the specifics of the question.
        (StatusCode::OK, Json(question)).into_response()
    } else if let Ok(None) = query_result {
        // If you cannot find the question ID, respond with an error.
        let error_message =
            json!({"error":"Question with this specific if not found or it doesn't exists!"});
        (StatusCode::NOT_FOUND, Json(error_message)).into_response()
    } else {
        // If there was an internal server error during retrieval, respond with an error.
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
    // Try removing the query from the database.
    let delete_result = sqlx::query("DELETE FROM questions_table WHERE question_id = $1")
        .bind(q_id)
        .execute(&*database_pool)
        .await;

    if let Ok(question_deleted) = delete_result {
        // Verify whether any rows were impacted—that is, whether the question was located and removed.
        if question_deleted.rows_affected() > 0 {
            // If the question was eliminated, successfully respond to it.
            let success_message = json!({"message":"Question deleted successfully"});
            (StatusCode::OK, Json(success_message)).into_response()
        } else {
            // If you cannot find the question ID, respond with an error.
            let error_message =
                json!({"error":"Question with this specific if not found or it doesn't exists!"});
            (StatusCode::NOT_FOUND, Json(error_message)).into_response()
        }
    } else {
        // If there was an internal server error during deletion, respond with an error.
        let error_message = json!({"error": "Internal server error"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
    }
}

/// One question or an array of questions can be sent as a JSON payload to this function.
/// After that, it makes an attempt to add the items to the database, and depending on whether the operation was successful or not, it returns the relevant replies.
///
/// # Arguments
/// * `State(database_pool)` - A shared reference to the connection pool of PostgresSQL databases.
/// * `Json(input)` - The JSON payload with the additional question or questions.
///
/// # Returns
/// * `StatusCode::CREATED` accompanied, if the questions are successfully added, by a success message.
/// * `StatusCode::BAD_REQUEST` when the input format is deemed invalid, in addition to an error message.
/// * If there are problems with the database insertion, provide appropriate error answers.
pub async fn add_questions(
    State(database_pool): State<Arc<PgPool>>,
    Json(input): Json<Value>,
) -> impl IntoResponse {
    // Verify whether the input is a series of questions.
    if let Some(questions) = input.as_array() {
        // Repeat for every question in the array, then add it to the database.
        for question in questions {
            if let Err(error) = insert_question(&database_pool, question).await {
                return error.into_response();
            }
        }
        // Successfully respond if all the questions are included.
        let success_message = json!({"message": "All questions added successfully"});
        (StatusCode::CREATED, Json(success_message)).into_response()
    } else if input.is_object() {
        // Respond to a single input question
        if let Err(error) = insert_question(&database_pool, &input).await {
            return error.into_response();
        }
        // Send a success message in response to a single query.
        let success_message = json!({"message": "Question added successfully"});
        (StatusCode::CREATED, Json(success_message)).into_response()
    } else {
        // If the question is added, successfully respond to it.
        let error_message = json!({"error": "Invalid input format. Expected a single object or an array of objects."});
        (StatusCode::BAD_REQUEST, Json(error_message)).into_response()
    }
}

/// A helper function for adding a solitary query to the database.
/// This function inserts data into the database and verifies the input.
///
/// # Arguments
/// * `database_pool` - A common reference to the connection pool for PostgresSQL.
/// * `question` - A reference to the JSON value that the query is represented by.
///
/// # Returns
/// * `Ok(())` whether the question is correctly inserted.
/// * If there are problems with validation or insertion, a JSON error message and a tuple with {StatusCode` are contained in `Err`.
async fn insert_question(
    database_pool: &Arc<PgPool>,
    question: &Value,
) -> Result<(), (StatusCode, Json<Value>)> {
    // Extract and verify the fields that are necessary from the inquiry.
    let question_title = question
        .get("question_title")
        .and_then(|value| value.as_str());
    let type_of_content = question
        .get("type_of_content")
        .and_then(|value| value.as_str());
    let type_of_question = question
        .get("type_of_question")
        .and_then(|value| value.as_array());

    // If every field is present and legitimate, proceed.
    if let (Some(question_title), Some(type_of_content), Some(type_of_question)) =
        (question_title, type_of_content, type_of_question)
    {
        // Convert type_of_question to a Vec<String>
        let type_of_question: Vec<String> = type_of_question
            .iter()
            .filter_map(|type_of_question| type_of_question.as_str().map(String::from))
            .collect();

        // Try entering the query in the database.
        let insert_result = sqlx::query(
            "INSERT INTO questions_table (question_title, type_of_content, type_of_question) VALUES ($1, $2, $3)",
        )
            .bind(question_title)
            .bind(type_of_content)
            .bind(&type_of_question)
            .execute(&**database_pool)
            .await;

        // Respond to any insertion errors.
        if insert_result.is_err() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error during insertion of question"})),
            ));
        }
        Ok(())
    } else {
        // If any mandatory fields are missing or incorrect, respond with an error.
        Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid input for a question. Must be missing inputs"})),
        ))
    }
}

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
