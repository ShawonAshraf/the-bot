use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct HealthResponse {
    status: String,
}

pub async fn check_backend_health() -> Result<String, Error> {
    // Read backend URL from env variable
    let url = match env::var("BACKEND_URL") {
        Ok(url) => url,
        Err(_) => {
            return Ok("The backend URL couldn't be found in the environment variables. Please set the BACKEND_URL variable.".into());
        }
    };

    let resp = reqwest::get(&url).await?;

    if resp.status().is_success() {
        return if let Ok(data) = resp.json::<HealthResponse>().await {
            if data.status.to_lowercase() == "ok" {
                Ok("Backend is running ✅".into())
            } else {
                Ok(format!(
                    "Backend is NOT running ❌ — status: {}",
                    data.status
                ))
            }
        } else {
            Ok("Backend is NOT running ❌ — invalid JSON".into())
        };
    }

    Ok(format!(
        "Backend is NOT running ❌ — HTTP status: {}",
        resp.status()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::MockServer;
    use std::env;

    unsafe fn set_env_var(key: &str, value: &str) {
        unsafe {
            env::set_var(key, value);
        }
    }

    unsafe fn clear_env_var(key: &str) {
        unsafe {
            env::remove_var(key);
        }
    }

    #[tokio::test]
    async fn test_backend_health_all_scenarios() {
        // Test 1: Missing environment variable
        unsafe { clear_env_var("BACKEND_URL"); }
        
        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "The backend URL couldn't be found in the environment variables. Please set the BACKEND_URL variable.");

        // Test 2: Success with OK status
        let server1 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/health", server1.base_url())); }
        
        let mock_ok = server1.mock(|when, then| {
            when.method("GET")
                .path("/health");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({"status": "ok"}));
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is running ✅");
        mock_ok.assert();
        unsafe { clear_env_var("BACKEND_URL"); }

        // Test 3: Success with invalid JSON
        let server2 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/invalid", server2.base_url())); }
        
        let mock_invalid = server2.mock(|when, then| {
            when.method("GET")
                .path("/invalid");
            then.status(200)
                .header("content-type", "application/json")
                .body("{invalid json}");
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is NOT running ❌ — invalid JSON");
        mock_invalid.assert();
        unsafe { clear_env_var("BACKEND_URL"); }

        // Test 4: Non-OK status
        let server3 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/error", server3.base_url())); }
        
        let mock_error = server3.mock(|when, then| {
            when.method("GET")
                .path("/error");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({"status": "error"}));
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is NOT running ❌ — status: error");
        mock_error.assert();
        unsafe { clear_env_var("BACKEND_URL"); }

        // Test 5: HTTP 500 error
        let server4 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/500", server4.base_url())); }
        
        let mock_500 = server4.mock(|when, then| {
            when.method("GET")
                .path("/500");
            then.status(500);
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is NOT running ❌ — HTTP status: 500 Internal Server Error");
        mock_500.assert();
        unsafe { clear_env_var("BACKEND_URL"); }

        // Test 6: HTTP 404 error
        let server5 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/404", server5.base_url())); }
        
        let mock_404 = server5.mock(|when, then| {
            when.method("GET")
                .path("/404");
            then.status(404);
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is NOT running ❌ — HTTP status: 404 Not Found");
        mock_404.assert();
        unsafe { clear_env_var("BACKEND_URL"); }

        // Test 7: Case insensitive OK status (uppercase)
        let server6 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/uppercase", server6.base_url())); }
        
        let mock_uppercase = server6.mock(|when, then| {
            when.method("GET")
                .path("/uppercase");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({"status": "OK"}));
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is running ✅");
        mock_uppercase.assert();
        unsafe { clear_env_var("BACKEND_URL"); }

        // Test 8: Case insensitive OK status (mixed case)
        let server7 = MockServer::start();
        unsafe { set_env_var("BACKEND_URL", &format!("{}/mixed", server7.base_url())); }
        
        let mock_mixed = server7.mock(|when, then| {
            when.method("GET")
                .path("/mixed");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(serde_json::json!({"status": "Ok"}));
        });

        let result = check_backend_health().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Backend is running ✅");
        mock_mixed.assert();
        unsafe { clear_env_var("BACKEND_URL"); }
    }
}

