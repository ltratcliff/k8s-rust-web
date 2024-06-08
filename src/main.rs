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
        .route("/health", get(get_health))
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
    tracing::info!("GET /api");
    let envs: HashMap<String, String> = env::vars().collect();
    Json(envs)
}


/// Retrieves the health status of the application.
///
/// This function returns the HTTP status code and a static string indicating the health status of the application.
/// The status code will be `StatusCode::OK` (200) if the application is healthy, and the string will be "OK".
/// This function is intended to be used for health checks, such as by a load balancer or monitoring system.
async fn get_health() -> (StatusCode, &'static str) {
    tracing::info!("GET /health");
    (StatusCode::OK, "OK")
}


/// The HTML template used by the application.
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
        <h2 style="margin:5px">Local IP Address</h2>
        <p>{{envs.LOCAL_IP}}</p>
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


#[cfg(test)]
mod tests {
    use std::io::Bytes;
    use super::*;
    use axum::body::Body;
    use axum::http::{header, Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_root() {
        let app = Router::new().route("/", get(root));

        let response = app
            .into_service()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/html; charset=utf-8"
        );
    }

    #[tokio::test]
    async fn test_get_env() {
        let app = Router::new().route("/api", get(get_env));

        let response = app
            .oneshot(Request::builder().uri("/api").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn test_get_health() {
        let app = Router::new().route("/health", get(get_health));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

}