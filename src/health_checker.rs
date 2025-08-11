use rand::Rng;
use reqwest::Error;
use serde::Deserialize;
use std::env;
use tracing::info;

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
    let (emoji, status_text) = if status.to_lowercase() == "ok" {
        ("🚀", "Service Healthy")
    } else {
        ("⚠️", "Service Down")
    };

    format!(
        "```\n{} | {}\n\n{} has been checked\n\nEnvironment          Status\n{}                 {}\n\nHealth Check\n{} {}\n```",
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

fn send_funny() -> String {
    // long replies
    let lmaos: Vec<&str> = [
        "এই মেসেজ কেডায় দিসে? 🤬",
        "আর কাম কাজ নাই? 🥴",
        "পুৎ কইরা দিমু 😈",
        "স্বজন হারানোর বেদনা আমিও বুঝি 😭",
        "আহো ভাতিজা আহো 😈",
        "আমি জুনায়েদ 😇",
        "সাগর, তুমি ভালো হয়ে যাও, মাসুদ হয়নি, তুমি হউ। 🥸",
        "ইংরেজিতে যেহেতু বুইলছেন, ঠিকই হবে! 🤓",
        "চ্যালেঞ্জিং টাইমস! 😎",
        "১০% নিয়া গেলো লন্ডনের ই বাসে রে, মরার কোকিলে! 🐦‍⬛",
    ]
    .to_vec();

    // pick a random message from lmaos
    // make the random selection using rand::Rng
    let random_index = rand::rng().random_range(0..lmaos.len());
    let lmao_msg = lmaos[random_index];
    lmao_msg.to_string()
}

fn parse_message(message: &str) -> Result<ParsedData, String> {
    // Split the message into parts
    let parts: Vec<&str> = message.split_whitespace().collect();

    // Check if the message has at least 3 parts
    if parts.len() != 3 {
        let funny_response = send_funny();
        return Err(funny_response.as_str().into());
    }

    // Extract service and environment
    let service = parts[1].to_string();
    let environment = parts[2].to_string();

    if service != "backend" && service != "frontend" {
        return Err(send_funny().as_str().into());
    }

    Ok(ParsedData {
        service,
        environment,
    })
}

pub async fn check_health(message: String) -> Result<String, Error> {
    info!("Received health check request: {}", message);
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
            return Ok("এই এন্ডপয়েন্টের কোন হদিস পাইলাম না! 😅".to_string());
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
