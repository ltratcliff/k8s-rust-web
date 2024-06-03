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


async fn root() -> (StatusCode, Html<String>) {
    tracing::info!("GET /");
    env::set_var("HOSTNAME", gethostname::gethostname().to_string_lossy().to_string());
    let my_local_ip = local_ip().unwrap();
    env::set_var("LOCAL_IP", my_local_ip.to_string());
    let envs: HashMap<String, String> = env::vars().collect();
    let rendered = render!(HTML, envs);
    
    (StatusCode::OK, Html(rendered.to_string()))
}

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
