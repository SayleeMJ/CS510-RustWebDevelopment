use lazy_static::lazy_static;
use std::sync::RwLock;

/// Defines the structure and functionalities for managing questions
pub mod questions_module {

    use serde::{Deserialize, Serialize};

    /// Represents a single question in the database
    /// Each question includes an ID, title, content type and a list of question categories
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Question {
        pub question_id: String,
        pub question_title: String,
        pub type_of_content: String,
        pub type_of_question: Vec<String>,
    }
}

lazy_static! {
    pub static ref QUESTIONS_DATABASE: RwLock<Vec<questions_module::Question>> =
        RwLock::new(Vec::new());
}

/// Initializes the database into JSON format
pub fn initialize_questions_database() {
    let mut questions_data = QUESTIONS_DATABASE.write().unwrap();
    questions_data.push(questions_module::Question {
        question_id: "1".to_string(),
        question_title: "Hello".to_string(),
        type_of_content: "How are you?".to_string(),
        type_of_question: vec!["Greeting".to_string()],
    });

    questions_data.push(questions_module::Question {
        question_id: "2".to_string(),
        question_title: "About work".to_string(),
        type_of_content: "What do you do?.".to_string(),
        type_of_question: vec!["Greeting".to_string()],
    });

    questions_data.push(questions_module::Question {
        question_id: "3".to_string(),
        question_title: "Hobby".to_string(),
        type_of_content: "What do you like to do in your free time?.".to_string(),
        type_of_question: vec!["Greeting".to_string()],
    });
}
