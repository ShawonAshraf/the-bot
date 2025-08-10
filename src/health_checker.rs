use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct HealthResponse {
    status: String,
}

#[derive(Deserialize, Debug)]
struct ParsedData {
    service: String,
    environment: String,
}

fn parse_message(message: &str) -> Result<ParsedData, String> {
    // Split the message into parts
    let parts: Vec<&str> = message.split_whitespace().collect();

    // Check if the message has at least 3 parts
    if parts.len() < 3 {
        return Err(
            "Invalid message format. Expected format: '!health <service> <environment>'".into(),
        );
    }

    // Extract service and environment
    let service = parts[1].to_string();
    let environment = parts[2].to_string();

    Ok(ParsedData {
        service,
        environment,
    })
}

pub async fn check_health(message: String) -> Result<String, Error> {
    // parse the message
    let parsed = match parse_message(&message) {
        Ok(data) => data,
        Err(e) => return Ok(e),
    };

    // find the url var from env
    let url_var = format!(
        "{}_{}_URL",
        parsed.service.to_uppercase(),
        parsed.environment.to_uppercase()
    );
    let url = match env::var(&url_var) {
        Ok(url) => url,
        Err(_) => {
            return Ok(format!(
                "The URL for {} in {} environment couldn't be found in the environment variables. Please set the {} variable.",
                parsed.service, parsed.environment, url_var
            ));
        }
    };

    // Make a GET request to the backend URL
    let resp = reqwest::get(&url).await?;

    // Check if the response status is successful and then return the health status
    if resp.status().is_success() {
        return if let Ok(data) = resp.json::<HealthResponse>().await {
            if data.status.to_lowercase() == "ok" {
                let msg = format!(
                    "{} :: {} is running ✅ - status: {}",
                    parsed.service, parsed.environment, data.status
                );
                Ok(msg.into())
            } else {
                Ok(format!(
                    "{} :: {} NOT running ❌ — status: {}",
                    parsed.service,
                    parsed.environment,
                    data.status
                ))
            }
        } else {
            Ok("Invalid JSON response from the endpoint".into())
        };
    }

    Ok(format!(
        "{} :: {} NOT running ❌ — HTTP status: {}",
        parsed.service,
        parsed.environment,
        resp.status()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use mockito;
    use std::env;

    #[test]
    fn test_parse_message_valid() {
        let message = "!health myservice production";
        let result = parse_message(message).unwrap();
        
        assert_eq!(result.service, "myservice");
        assert_eq!(result.environment, "production");
    }

    #[test]
    fn test_parse_message_insufficient_parts() {
        let message = "!health myservice";
        let result = parse_message(message);
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid message format. Expected format: '!health <service> <environment>'"
        );
    }

    #[test]
    fn test_parse_message_empty() {
        let message = "";
        let result = parse_message(message);
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_health_invalid_message() {
        let message = "!health incomplete".to_string();
        let result = check_health(message).await.unwrap();
        
        assert_eq!(
            result,
            "Invalid message format. Expected format: '!health <service> <environment>'"
        );
    }

    #[tokio::test]
    async fn test_check_health_missing_env_var() {
        unsafe { env::remove_var("TESTSERVICE_STAGING_URL"); }
        
        let message = "!health testservice staging".to_string();
        let result = check_health(message).await.unwrap();
        
        assert!(result.contains("The URL for testservice in staging environment couldn't be found"));
        assert!(result.contains("TESTSERVICE_STAGING_URL"));
    }

    #[tokio::test]
    async fn test_check_health_success_with_httpmock() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({"status": "ok"}));
        });

        unsafe { env::set_var("MYAPP_PROD_URL", &server.url("/health")); }
        
        let message = "!health myapp prod".to_string();
        let result = check_health(message).await.unwrap();
        
        mock.assert();
        assert!(result.contains("myapp :: prod is running ✅"));
        assert!(result.contains("status: ok"));

        unsafe { env::remove_var("MYAPP_PROD_URL"); }
    }

    #[tokio::test]
    async fn test_check_health_unhealthy_status_with_httpmock() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({"status": "error"}));
        });

        unsafe { env::set_var("WEBAPP_DEV_URL", &server.url("/health")); }
        
        let message = "!health webapp dev".to_string();
        let result = check_health(message).await.unwrap();
        
        mock.assert();
        assert!(result.contains("webapp :: dev NOT running ❌"));
        assert!(result.contains("status: error"));

        unsafe { env::remove_var("WEBAPP_DEV_URL"); }
    }

    #[tokio::test]
    async fn test_check_health_invalid_json_with_httpmock() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(200)
                .header("content-type", "application/json")
                .body("invalid json");
        });

        unsafe { env::set_var("API_TEST_URL", &server.url("/health")); }
        
        let message = "!health api test".to_string();
        let result = check_health(message).await.unwrap();
        
        mock.assert();
        assert_eq!(result, "Invalid JSON response from the endpoint");

        unsafe { env::remove_var("API_TEST_URL"); }
    }

    #[tokio::test]
    async fn test_check_health_http_error_with_httpmock() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/health");
            then.status(500);
        });

        unsafe { env::set_var("BACKEND_STAGING_URL", &server.url("/health")); }
        
        let message = "!health backend staging".to_string();
        let result = check_health(message).await.unwrap();
        
        mock.assert();
        assert!(result.contains("backend :: staging NOT running ❌"));
        assert!(result.contains("HTTP status: 500"));

        unsafe { env::remove_var("BACKEND_STAGING_URL"); }
    }

    #[tokio::test]
    async fn test_check_health_success_with_mockito() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/health")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status": "ok"}"#)
            .create_async()
            .await;

        unsafe { env::set_var("SERVICE_PROD_URL", &format!("{}/health", server.url())); }
        
        let message = "!health service prod".to_string();
        let result = check_health(message).await.unwrap();
        
        mock.assert_async().await;
        assert!(result.contains("service :: prod is running ✅"));

        unsafe { env::remove_var("SERVICE_PROD_URL"); }
    }

    #[tokio::test]
    async fn test_check_health_404_with_mockito() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/health")
            .with_status(404)
            .create_async()
            .await;

        unsafe { env::set_var("NOTFOUND_DEV_URL", &format!("{}/health", server.url())); }
        
        let message = "!health notfound dev".to_string();
        let result = check_health(message).await.unwrap();
        
        mock.assert_async().await;
        assert!(result.contains("notfound :: dev NOT running ❌"));
        assert!(result.contains("HTTP status: 404"));

        unsafe { env::remove_var("NOTFOUND_DEV_URL"); }
    }
}