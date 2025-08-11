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

fn format_response(service: &str, environment: &str, status: &str) -> String {
    let (emoji, status_text, color_bar) = if status.to_lowercase() == "ok" {
        ("🚀", "Service Healthy", "```diff\n+ ")
    } else {
        ("⚠️", "Service Down", "```diff\n- ")
    };

    format!(
        "{}```\n{} | {}\n\n{} has been checked\n\n**Environment**           **Status**\n{}                    {}\n\n**Health Check**\n{} {}\n```",
        color_bar,
        emoji,
        status_text,
        service,
        environment,
        status.to_uppercase(),
        if status.to_lowercase() == "ok" {
            "✅"
        } else {
            "❌"
        },
        if status.to_lowercase() == "ok" {
            "Healthy"
        } else {
            "Unhealthy"
        }
    )
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
                let msg = format_response(&parsed.service, &parsed.environment, &data.status);
                Ok(msg.into())
            } else {
                Ok(format_response(
                    &parsed.service,
                    &parsed.environment,
                    &data.status,
                ))
            }
        } else {
            Ok("Invalid JSON response from the endpoint".into())
        };
    }

    Ok(format_response(
        &parsed.service,
        &parsed.environment,
        &resp.status().to_string(),
    ))
}
