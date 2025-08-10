use reqwest::Error;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct HealthResponse {
    status: String,
}

pub async fn check_backend_health() -> Result<String, Error> {
    // Read backend URL from env variable
    let url = env::var("BACKEND_URL").unwrap_or_else(|_| {
        "The backend URL couldn't be found in the environment variables. Please set the BACKEND_URL variable.".into()
    });

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
