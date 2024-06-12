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
        throw new Error('Network response was not ok');
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
        console.error('Failed to fetch questions:', error);
    }
}

/**
 *  This asynchronous function retrieves a question by its ID from the backend API via a GET call to the
 *  '/getQuestionByID/:id' endpoint.
 *  If the request is successful, it evaluates the JSON response and executes the displayQuestionDetail method to show the question details.
 *  If the request fails, it sends an error message to the console.
 */
async function fetchQuestionById() {
    const questionId = document.getElementById('questionIdInput').value;
    if (!questionId) {
        alert("Please enter a question ID.");
        return;
    }

    try {
        const questionDetails = await fetchJsonFormat(`/getQuestionByID/${questionId}`);
        displayQuestionDetail(questionDetails);
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
function generateQuestionHTML(question) {
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
        listItem.innerHTML = generateQuestionHTML(question);
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
function displayQuestionDetail(questionDetail) {
    const questionDetailElement = document.getElementById('questionDetail');

    if (questionDetail.error) {
        questionDetailElement.innerHTML = `<div class="error">${questionDetail.error}</div>`;
    } else {
        questionDetailElement.innerHTML = generateQuestionHTML(questionDetail);
    }

    questionDetailElement.classList.add('visible');
}

// Add an event listener that calls fetchAllQuestions when the DOM content is loaded.
document.addEventListener('DOMContentLoaded', fetchAllQuestions);

// Add an event listener for the fetch button
document.getElementById('fetchQuestionButton').addEventListener('click', fetchQuestionById);
