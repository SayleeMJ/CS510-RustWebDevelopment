/**
 *  This asynchronous method does a GET request to the specified URL and provides a JSON response.
 *  If the request fails, it returns an error with a message.
 *
 *  @param {string} url - The GET request should be sent to this URL.
 *  @returns {Promise<Object>} - The backend API returned a JSON answer.
 */
async function fetchJsonFormat(url) {
    const jsonResponse = await fetch(url);

    if (!jsonResponse.ok) {
        throw new Error('Error occurred  in network response');
    }

    return await jsonResponse.json();
}

/**
 *  This asynchronous function retrieves questions from the backend API via a GET call to the
 *  '/allQuestions' endpoint.
 *  If the request is successful, it evaluates the JSON response and executes the displayAllQuestions method to show the questions.
 *  If the request fails, it sends an error message to the console.
 */
async function fetchAllQuestions() {
    try {
        const allQuestions = await fetchJsonFormat('/allQuestions');
        displayAllQuestions(allQuestions);
    } catch (error) {
        console.error('Failed to fetch questions from database:', error);
    }
}

/**
 *  This asynchronous function retrieves a question by its ID from the backend API via a GET call to the
 *  '/getQuestionByID/:id' endpoint.
 *  If the request is successful, it evaluates the JSON response and executes the displayQuestionById method to show the question details.
 *  If the request fails, it sends an error message to the console.
 */
async function fetchQuestionById(event) {
    event.preventDefault();
    const questionId = document.getElementById('questionIdInput').value;
    if (!questionId) {
        alert("Please enter a question ID.");
        return;
    }

    try {
        const questionDetails = await fetchJsonFormat(`/getQuestionByID/${questionId}`);
        displayQuestionById(questionDetails);
    } catch (error) {
        console.error('Failed to fetch question:', error);
    }
}

/**
 * Function that generates HTML content for a query.
 *
 * @param {Object} question - The question object contains its details.
 * @returns {string} - HTML content for the question.
 */
function htmlFormat(question) {
    return `
        <div class="id">Question ID: ${question.question_id}</div>
        <div class="title">Question Title: ${question.question_title}</div>
        <div class="content-type">Type of Content: ${question.type_of_content}</div>
        <div class="question-types">Type of Question: ${question.type_of_question.join(', ')}</div>
    `;
}

/**
 * Function for displaying all questions in the DOM.
 *
 * This function generates list items for each question in an array
 * and adds them to the 'allQuestions' element.
 *
 * @param {Object} allQuestions - Array of question objects
 */
function displayAllQuestions(allQuestions) {
    const questionsIDElement = document.getElementById('allQuestions');
    questionsIDElement.innerHTML = '';

    allQuestions.forEach(question => {
        const listItem = document.createElement('li');
        listItem.innerHTML = htmlFormat(question);
        questionsIDElement.appendChild(listItem);
    });
}

/**
 * Function for displaying the details of a single question in the DOM.
 *
 * This function creates elements to show the details of a question and adds them to the 'questionDetail' element.
 *
 * @param {Object} questionDetail - The question object containing its details
 */
function displayQuestionById(questionDetail) {
    const questionDetailElement = document.getElementById('questionDetail');

    if (questionDetail.error) {
        questionDetailElement.innerHTML = `<div class="error">${questionDetail.error}</div>`;
    } else {
        questionDetailElement.innerHTML = htmlFormat(questionDetail);
    }

    questionDetailElement.classList.add('visible');
}

/**
 * This asynchronous function makes a POST call to the backend API to add a new question.
 * The question information are gathered from the form inputs.
 *
 * @param {Event} event - The form submission event.
 */
async function addNewQuestion(event) {
    event.preventDefault();

    // Obtain values from the form inputs.
    const questionTitle = document.getElementById('questionTitle').value;
    const typeOfContent = document.getElementById('typeOfContent').value;
    const typeOfQuestion = document.getElementById('typeOfQuestion').value.split(',').map(str => str.trim());

    const newQuestion = {
        question_title: questionTitle,
        type_of_content: typeOfContent,
        type_of_question: typeOfQuestion
    };

    try {
        // To add the new question, send a POST request to the backend.
        const json_response = await fetch('/addQuestion', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(newQuestion)
        });

        // Check if the json_response is not okay, and throw an error if it is.
        if (!json_response.ok) {
            throw new Error('Error occurred in network json_response');
        }

        // Parse the server's JSON answer.
        const response_result = await json_response.json();

        // Send a message to the user from the server.
        alert(response_result.message);

        // Reset form inputs.
        document.getElementById('addNewQuestionForm').reset();

        // To refresh the list of questions, fetch all questions again.
        await fetchAllQuestions();
    } catch (error) {

        // If the request fails, log the error in the console and notify the user.
        console.error('Failed to add a new question:', error);
        alert('Failed to add a new question. Please try again!');
    }
}

/**
 * Function that handles navigation and displays the corresponding section.
 */
function handlePageNavigation(event) {
    event.preventDefault();

    // Take the 'active' class out of all sections.
    document.querySelectorAll('.section').forEach(section => {
        section.classList.remove('active');
    });

    // Take the 'active' class out of all sections.
    const targetSection = document.querySelector(event.target.getAttribute('href'));
    targetSection.classList.add('active');
}

// Configure event listeners for the navigation links.
document.querySelectorAll('.nav-link').forEach(link => {
    link.addEventListener('click', handlePageNavigation);
});

// Display the default section when the page loads.
document.addEventListener('DOMContentLoaded', () => {
    fetchAllQuestions().then(() => "Could not find questions in database!"); // Fetch all questions on page load
    document.querySelector('#fetchAllQuestionsSection').classList.add('active'); // Show the default section
});

// Add an event listener for the fetch button
document.getElementById('fetchQuestionButton').addEventListener('click', fetchQuestionById);

// Add an event listener for the add question form
document.getElementById('addNewQuestionForm').addEventListener('submit', addNewQuestion);