use std::{collections::HashMap, env};

use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
    response::Html,
};

use minijinja::render;

use local_ip_address::local_ip;

#[tokio::main]
/// The main entry point for the application.
///
/// This function sets up the Axum web server, configures the routes, and starts
/// the server to listen on port 3000. The server will handle the following routes:
///
/// - `GET /`: Renders an HTML page with environment variables.
/// - `GET /health`: Returns a simple "OK" response.
/// - `GET /api`: Returns a JSON response with all the environment variables.
///
/// The server uses the `tracing` crate for logging, and the `minijinja` crate for
/// rendering the HTML template.
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/health", get(|| async { (StatusCode::OK, "OK") }))
        .route("/api", get(get_env));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


/// Renders the root page of the application.
///
/// This function sets the `HOSTNAME` and `LOCAL_IP` environment variables, collects
/// all the environment variables into a `HashMap`, and then renders an HTML template
/// using the `minijinja` crate. The rendered HTML is returned with a `StatusCode::OK`.
///
/// # Returns
/// A tuple containing the `StatusCode::OK` and the rendered HTML as a `String`.
async fn root() -> (StatusCode, Html<String>) {
    tracing::info!("GET /");
    env::set_var("HOSTNAME", gethostname::gethostname().to_string_lossy().to_string());
    let my_local_ip = local_ip().unwrap();
    env::set_var("LOCAL_IP", my_local_ip.to_string());
    let envs: HashMap<String, String> = env::vars().collect();
    let rendered = render!(HTML, envs);
    
    (StatusCode::OK, Html(rendered.to_string()))
}

/// Returns a JSON response containing all the environment variables as a HashMap.
///
/// This function collects all the environment variables into a HashMap and returns
/// them as a JSON response. This can be used to retrieve information about the
/// runtime environment of the application.
///
/// # Returns
/// A JSON response containing a HashMap of all the environment variables.
async fn get_env() -> Json<HashMap<String, String>> {
    let envs: HashMap<String, String> = env::vars().collect();
    Json(envs)
}

const HTML: &'static str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Welcome {{envs.HOSTNAME}}</title>
    <!-- Add Bootstrap CSS link -->
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css">
</head>
<body>
    <div class="container text-center">
        <h1>Welcome to <span class="rainbow">{{envs.HOSTNAME}}</span></h1>
    </div>
    <div>
        <h2 style="margin:5px">Environment Variables</h2>
        <table class="table">
            <thead>
                <tr>
                    <th>Key</th>
                    <th>Value</th>
                </tr>
            </thead>
            <tbody>
                {% for item in envs %}
                <tr>
                    <td>{{ item }}</td>
                    <td>{{ envs[item] }}</td>
                </tr>
                {% endfor %}
            </tbody>
        </table>
    </div>

    <!-- Add Bootstrap JS scripts -->
    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js"></script>
</body>
<script>
    // Rainbow text easing
    var colors = ['#FF0000', '#FF7F00', '#FFFF00', '#00FF00', '#0000FF', '#4B0082', '#9400D3'];
    var i = 0;
    setInterval(function() {
        document.querySelector('.rainbow').style.color = colors[i];
        i = (i + 1) % colors.length;
        document.querySelector('.rainbow').style.transition = 'color 2s';
        document.querySelector('.rainbow').style.transitionTimingFunction = 'ease';
        document.querySelector('.rainbow').style.transitionDuration = '2s';
        document.querySelector('.rainbow').style.transitionDelay = '0s';
    }, 1000);

</script>
</html>
"#;
