use axum::response::{Html, IntoResponse, Response};
use axum::http::{Request, StatusCode};
use axum::body::Body;
use std::convert::Infallible;
use crate::admin::routes_names::RouteNames;

// The 404 handler
use axum::response::{Html, IntoResponse, Response};
use axum::http::{Request, StatusCode};
use axum::body::Body;
use std::convert::Infallible;
use crate::admin::routes_names::RouteNames;

// The 404  
async fn handle_404() -> impl IntoResponse {

 let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - Page Not Found</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            height: 100vh;
            margin: 0;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            background-color: #f7f7f7;
            color: #333;
        }
        .container {
            text-align: center;
            padding: 2rem;
        }
        h1 {
            font-size: 8rem;
            margin: 0;
            color: #e74c3c;
        }
        h2 {
            font-size: 2rem;
            margin: 0;
            margin-bottom: 1rem;
        }
        p {
            font-size: 1.2rem;
            margin-bottom: 2rem;
        }
        a {
            color: #3498db;
            text-decoration: none;
            font-weight: bold;
        }
        a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>404</h1>
        <h2>Page Not Found</h2>
        <p>The page you are looking for might have been removed, had its name changed, or is temporarily unavailable.</p>
        <a href="/">Go Back Home</a>
    </div>
</body>
</html>
"#;

    (StatusCode::NOT_FOUND, Html(html))
}