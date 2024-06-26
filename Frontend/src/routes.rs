use warp::{http::Response, Filter};

use crate::route_handlers::{
    add_new_question, delete_question_by_id, retrieve_all_questions, retrieve_question_by_id,
    update_question_by_id,
};

/// Utility function for formatting the API response.
fn api_format_response(response_body: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(response_body)
        .unwrap())
}

/// Create a route to serve static files (HTML, CSS, and JavaScript).
fn static_file_routes(
    path: &'static str,
    file: &'static str,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(path).and(warp::fs::file(file))
}

/// Configure the routes to serve static files and API endpoints.
///
/// # Returns
/// A combination filter that covers all routes.
pub fn create_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Serve static files
    let html_route = warp::path::end().and(warp::fs::file("templates/index.html"));
    let css_route = static_file_routes("index.css", "templates/index.css");
    let js_route = static_file_routes("index.js", "templates/index.js");

    // API routes for fetching questions
    let fetch_all_questions_route = warp::path("allQuestions")
        .and_then(retrieve_all_questions)
        .and_then(|response_body| async { api_format_response(response_body) });

    // API route for fetching questions by ID
    let fetch_question_by_id_route = warp::path!("getQuestionByID" / i32)
        .and_then(retrieve_question_by_id)
        .and_then(|response_body| async { api_format_response(response_body) });

    // API route for adding a new question
    let add_new_question_route = warp::path("addQuestion")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_new_question);

    // API route for updating an existing question
    let update_questions_route = warp::path!("updateQuestion" / i32)
        .and(warp::patch())
        .and(warp::body::json())
        .and_then(update_question_by_id);

    // API route for deleting a question
    let delete_question_route = warp::path!("deleteQuestion" / i32)
        .and(warp::delete())
        .and_then(delete_question_by_id);

    // Combine all routes
    html_route
        .or(fetch_all_questions_route)
        .or(fetch_question_by_id_route)
        .or(add_new_question_route)
        .or(update_questions_route)
        .or(delete_question_route)
        .or(css_route)
        .or(js_route)
}
