use crate::questions_database::questions_module::QuestionStructure;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;

/// Retrieves every question from the database.
///
/// # Arguments
/// * `database_pool` - A state where the database pool is located
///
/// # Returns
/// A list of every query in the database contained in a JSON answer
pub async fn fetch_all_questions(State(database_pool): State<Arc<PgPool>>) -> impl IntoResponse {
    // Attempt to fetch all questions from the database
    let all_questions = sqlx::query_as::<_, QuestionStructure>(
        "SELECT * FROM questions_table ORDER BY question_id",
    )
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

/// Updates a question in the database.
///
/// # Arguments
/// * `q_id` - The question's ID to update
/// * `database_pool` - A state that contains the database connection pool
/// * `Json(payload)` - A JSON payload with the most recent question details
///
/// # Returns
/// An error message will appear if the update is unsuccessful; otherwise, a success message.
pub async fn update_question(
    Path(q_id): Path<i32>,
    State(database_pool): State<Arc<PgPool>>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    // Retrieve the existing question from the database
    let select_query = sqlx::query_as::<_, QuestionStructure>(
        "SELECT * FROM questions_table WHERE question_id = $1",
    )
    .bind(q_id)
    .fetch_optional(&*database_pool)
    .await;
    // Verify whether the question was properly obtained.
    if let Ok(question) = select_query {
        // Check if the question exists
        if let Some(mut existing_question) = question {
            // If available, update the question fields with the payload data.
            if let Some(question_title) = payload
                .get("question_title")
                .and_then(|value| value.as_str())
            {
                existing_question.question_title = question_title.to_string();
            }
            if let Some(type_of_content) = payload
                .get("type_of_content")
                .and_then(|value| value.as_str())
            {
                existing_question.type_of_content = type_of_content.to_string();
            }
            if let Some(type_of_question) = payload
                .get("type_of_question")
                .and_then(|value| value.as_array())
            {
                existing_question.type_of_question = type_of_question
                    .iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect();
            }

            // Update the question in the database
            let update_query = sqlx::query(
                "UPDATE questions_table SET question_title = $1, type_of_content = $2, type_of_question = $3 WHERE question_id = $4",
            )
                .bind(&existing_question.question_title)
                .bind(&existing_question.type_of_content)
                .bind(&existing_question.type_of_question)
                .bind(q_id)
                .execute(&*database_pool)
                .await;

            // Verify if the query update was successful.
            if update_query.is_ok() {
                let success_message = json!({"message": "Question updated successfully"});
                (StatusCode::OK, Json(success_message)).into_response()
            } else {
                // Take action if the update query is unsuccessful.
                let error_message =
                    json!({"error": "Internal server error during update of question"});
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
            }
        } else {
            // Address the situation in which there is no question
            let error_message =
                json!({"error":"Question with this specific ID not found or it doesn't exist!"});
            (StatusCode::NOT_FOUND, Json(error_message)).into_response()
        }
    } else {
        // Address the situation where the question cannot be retrieved.
        let error_message = json!({"error": "Internal server error"});
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message)).into_response()
    }
}
