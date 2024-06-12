use reqwest;
use serde::{Deserialize, Serialize};
use warp::reject::Reject;
use warp::Rejection;

#[derive(Debug)]
pub struct RetrieveError;
impl Reject for RetrieveError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewQuestion {
    question_title: String,
    type_of_content: String,
    type_of_question: Vec<String>,
}

/// Helper function for sending a GET call to the backend API and returning the response as a string.
///
/// # Parameters
/// - `url`: The URL for the backend API endpoint.
///
/// # Returns
/// A string-formatted JSON response or a Warp rejection.
async fn fetch_from_backend_api(url: &str) -> Result<String, Rejection> {
    let json_response_result = reqwest::get(url).await;

    if json_response_result.is_err() {
        return Err(warp::reject::custom(RetrieveError));
    }

    let json_response = json_response_result.unwrap();

    let json_data_result = json_response.text().await;

    if json_data_result.is_err() {
        return Err(warp::reject::custom(RetrieveError));
    }

    let parsed_json_data = json_data_result.unwrap();

    Ok(parsed_json_data)
}

/// Returns the JSON result from an HTTP call sent to the backend API.
///
/// # Parameters
/// - `url`: The URL of the backend API endpoint.
/// - `method`: The HTTP method to employ for the request (such as GET, POST, or PATCH).
/// - `body`: The optional body for the request, serialized as JSON.
///
/// # Returns
/// A result containing the backend APIs JSON answer or a Warp refusal.
async fn send_request_to_backend_api(url: &str, method: reqwest::Method, body: Option<&impl Serialize>) -> Result<serde_json::Value, Rejection>{

    // Use the reqwest library to create a new HTTP client.
    let http_client = reqwest::Client::new();

    // Build the request using the supplied method and URL.
    let mut request_builder = http_client.request(method, url);

    // If a body is provided, serialize it into JSON and include it in the request.
    if let Some(body_data) = body {
        request_builder = request_builder.json(body_data);
    }

    // Send the request, mapping any errors to a specific RetrieveError.
    let json_response = request_builder.send().await.map_err(|_| warp::reject::custom(RetrieveError))?;
    let json_data = json_response.json::<serde_json::Value>().await.map_err(|_| warp::reject::custom(RetrieveError))?;

    // Return the parsed JSON data
    Ok(json_data)
}

/// This function makes a GET request to the backend API to retrieve all questions.
///
/// # Returns
/// A string-formatted JSON response or a Warp rejection.
pub async fn retrieve_all_questions() -> Result<String, Rejection> {
    let backend_api_url = "http://localhost:1000/getAllQuestions";
    fetch_from_backend_api(backend_api_url).await
}

/// This function sends a GET request to the backend API to obtain a question by ID.
///
/// # Parameters
/// - `question_id`: The ID of the question to be retrieved.
///
/// # Returns
/// A string-formatted JSON response or a Warp rejection.
pub async fn retrieve_question_by_id(question_id: i32) -> Result<String, Rejection> {
    let backend_api_url = format!("http://localhost:1000/getQuestionByID/{}", question_id.to_string());
    fetch_from_backend_api(&backend_api_url).await
}

/// This function makes a POST request to the backend API to add a new question.
///
/// # Parameters
/// - `new_question`: The question information must be supplied.
///
/// # Returns
/// A JSON response or a Warp rejection.
pub async fn add_new_question(new_question: NewQuestion) -> Result<impl warp::Reply, Rejection> {
    let backend_api_url = "http://localhost:1000/addQuestion";
    let json_data = send_request_to_backend_api(backend_api_url, reqwest::Method::POST, Some(&new_question)).await?;
    Ok(warp::reply::json(&json_data))
}

/// Sends a PATCH request to the backend API to update an existing inquiry based on its ID.
///
/// # Parameters
/// - `question_id`: The question ID that has to be modified.
/// - `updated_question`: Updated question data.
///
/// # Returns
/// A Warp reply containing the backend APIs JSON response, or a Warp rejection.
///
/// # Errors
/// If the backend API request fails, this method returns a 'RetrieveError'.
pub async fn update_question_by_id(question_id: i32, updated_question: NewQuestion) -> Result<impl warp::Reply, Rejection> {
    // Create the backend API URL using the question ID.
    let backend_api_url = format!("http://localhost:1000/updateQuestion/{}", question_id.to_string());
    let json_data = send_request_to_backend_api(&backend_api_url, reqwest::Method::PATCH, Some(&updated_question)).await?;
    Ok(warp::reply::json(&json_data))
}

/// To delete a question by ID, sends a DELETE request to the backend API.
///
/// # Parameters
/// - `question_id`: The ID of the question that will be erased.
///
/// # Returns
/// A JSON answer or Warp rejection.
pub async fn delete_question_by_id(question_id: i32) -> Result<impl warp::Reply, Rejection> {
    let backend_api_url = format!("http://localhost:1000/deleteQuestion/{}", question_id.to_string());
    let json_data = send_request_to_backend_api(&backend_api_url, reqwest::Method::DELETE, None::<&()>).await?;
    Ok(warp::reply::json(&json_data))
}