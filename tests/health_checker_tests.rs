use the_bot::health_checker::check_health;
use mockito::Server;

fn set_env_for(service: &str, env_name: &str, url: &str) {
    let key = format!("{}_{}_URL", service.to_uppercase(), env_name.to_uppercase());
    // In Rust 2024 edition, setting env var is unsafe
    unsafe { std::env::set_var(&key, url) };
}

#[tokio::test]
async fn health_ok_returns_healthy_message() {
    let mut server = Server::new_async().await;
    let endpoint = "/health";
    let _m = server
        .mock("GET", endpoint)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{\"status\":\"ok\"}")
        .create_async()
        .await;

    let url = format!("{}{}", server.url(), endpoint);
    set_env_for("backend", "dev", &url);

    let res = check_health("!health backend dev".to_string()).await.unwrap();
    assert!(res.contains("Service Healthy"));
    assert!(res.contains("✅"));
    assert!(res.contains("backend"));
    assert!(res.contains("dev"));
}

#[tokio::test]
async fn health_non_ok_json_returns_unhealthy_message() {
    let mut server = Server::new_async().await;
    let endpoint = "/health";
    let _m = server
        .mock("GET", endpoint)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{\"status\":\"fail\"}")
        .create_async()
        .await;

    let url = format!("{}{}", server.url(), endpoint);
    set_env_for("backend", "staging", &url);

    let res = check_health("!health backend staging".to_string()).await.unwrap();
    assert!(res.contains("Service Down"));
    assert!(res.contains("❌"));
}

#[tokio::test]
async fn health_invalid_json_returns_error_message() {
    let mut server = Server::new_async().await;
    let endpoint = "/health";
    let _m = server
        .mock("GET", endpoint)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create_async()
        .await;

    let url = format!("{}{}", server.url(), endpoint);
    set_env_for("frontend", "prod", &url);

    let res = check_health("!health frontend prod".to_string()).await.unwrap();
    assert!(res.contains("Invalid JSON response"));
}

#[tokio::test]
async fn health_http_error_status_returns_down_message() {
    let mut server = Server::new_async().await;
    let endpoint = "/health";
    let _m = server
        .mock("GET", endpoint)
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body("{\"status\":\"error\"}")
        .create_async()
        .await;

    let url = format!("{}{}", server.url(), endpoint);
    set_env_for("backend", "qa", &url);

    let res = check_health("!health backend qa".to_string()).await.unwrap();
    assert!(res.contains("Service Down"));
}

#[tokio::test]
async fn missing_env_var_returns_hint_message() {
    unsafe { std::env::remove_var("BACKEND_DEV_URL") };
    let res = check_health("!health backend dev".to_string()).await.unwrap();
    assert!(res.contains("হদিস"));
}

#[tokio::test]
async fn invalid_format_returns_funny_message() {
    let res = check_health("!health backend".to_string()).await.unwrap();
    assert!(!res.is_empty());
}
