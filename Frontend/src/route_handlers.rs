use reqwest;
use warp::reject::Reject;
use warp::Rejection;
use serde::{Serialize, Deserialize};

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
    // Define the backend API URL to add a new question.
    let backend_api_url = "http://localhost:1000/addQuestion";

    // Use the reqwest library to create a new HTTP client.
    let client = reqwest::Client::new();

    // Send a POST request to the backend API using the new question data in JSON format.
    let json_response_result = client.post(backend_api_url)
        .json(&new_question)
        .send()
        .await;

    // Check to see if the response_result contains an error.
    if json_response_result.is_err() {
        return Err(warp::reject::custom(RetrieveError));
    }

    // Unwrap the response_result to obtain the actual response.
    let json_response = json_response_result.unwrap();

    // Parse the response body into JSON
    let json_data_result = json_response.json::<serde_json::Value>().await;

    // Check whether the data_result contains an error.
    if json_data_result.is_err() {
        return Err(warp::reject::custom(RetrieveError));
    }

    // Unwrap the data_result to obtain the parsed JSON data.
    let parsed_json_data = json_data_result.unwrap();

    // Return JSON data as a Warp JSON response.
    Ok(warp::reply::json(&parsed_json_data))
}