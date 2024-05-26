use std::{collections::HashMap, env};

use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
    response::Html,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/env", get(get_env));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> (StatusCode, Html<&'static str>) {
    tracing::info!("GET /");
    (StatusCode::OK, Html(&HTML))
}

async fn get_env() -> (StatusCode, Json<HashMap<String, String>>) {
    tracing::info!("GET /env");
    env::set_var("HOSTNAME", gethostname::gethostname().to_string_lossy().to_string());
    let envs: HashMap<String, String> = env::vars().collect();
    (StatusCode::OK, Json(envs))
}

const HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Welcome Webpage</title>
    <!-- Add Bootstrap CSS link -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css">
</head>
<body>
    <div class="container">
        <h1>Welcome Enviroment Page!</h1>
        <a href="/env">Check Enviroment</a>
    </div>

    <!-- Add Bootstrap JS scripts -->
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
"#;