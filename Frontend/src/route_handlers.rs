use reqwest;
use warp::reject::Reject;
use warp::Rejection;

#[derive(Debug)]
pub struct RetrieveError;
impl Reject for RetrieveError {}

/// Helper function for sending a GET call to the backend API and returning the response as a string.
///
/// # Parameters
/// - `url`: The URL for the backend API endpoint.
///
/// # Returns
/// A string-formatted JSON response or a Warp rejection.
async fn fetch_from_backend_api(url: &str) -> Result<String, Rejection> {
    let response_result = reqwest::get(url).await;

    if response_result.is_err() {
        return Err(warp::reject::custom(RetrieveError));
    }

    let response = response_result.unwrap();

    let data_result = response.text().await;

    if data_result.is_err() {
        return Err(warp::reject::custom(RetrieveError));
    }

    let data = data_result.unwrap();

    Ok(data)
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
    let backend_api_url = format!("http://localhost:1000/getQuestionByID/{}", question_id);
    fetch_from_backend_api(&backend_api_url).await
}
