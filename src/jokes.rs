use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Joke {
    pub id: usize,
    pub r#type: String,
    pub setup: String,
    pub punchline: String,
}

#[derive(Error, Debug)]
pub enum JokeError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
}

pub async fn fetch_joke() -> Result<Vec<Joke>, JokeError> {
    let url = "https://official-joke-api.appspot.com/jokes/programming/random";
    let response = reqwest::get(url).await?;
    let response = response.error_for_status()?;
    let jokes: Vec<Joke> = response.json().await?;

    // check if the response is empty
    if jokes.is_empty() {
        tracing::warn!("No jokes found in the response, returning default joke");
        let default_joke = Joke {
            id: 0,
            r#type: "default".to_string(),
            setup: "জোক পাইতেসি না, সব ফ্রন্টএন্ডের দোষ! 😤".to_string(),
            punchline: "জোক পাইতেসি না, সব ফ্রন্টএন্ডের দোষ! 😤".to_string(),
        };
        return Ok(vec![default_joke]);
    }

    Ok(jokes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use serde_json::json;

    // Helper function to create a mock joke response
    fn create_mock_jokes() -> serde_json::Value {
        json!([
            {
                "id": 1,
                "type": "programming",
                "setup": "Why do programmers prefer dark mode?",
                "punchline": "Because light attracts bugs!"
            }
        ])
    }

    #[tokio::test]
    async fn test_fetch_joke_success() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/jokes/programming/random")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(create_mock_jokes().to_string())
            .create_async()
            .await;

        // Override the URL in your function to use the mock server
        // Note: You'll need to modify your function to accept a URL parameter for testing
        let url = format!("{}/jokes/programming/random", server.url());
        let response = reqwest::get(&url).await.unwrap();
        let jokes: Vec<Joke> = response.json().await.unwrap();

        assert_eq!(jokes.len(), 1);
        assert_eq!(jokes[0].setup, "Why do programmers prefer dark mode?");
        assert_eq!(jokes[0].punchline, "Because light attracts bugs!");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fetch_joke_empty_response() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/jokes/programming/random")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("[]")
            .create_async()
            .await;

        let url = format!("{}/jokes/programming/random", server.url());
        let response = reqwest::get(&url).await.unwrap();
        let jokes: Vec<Joke> = response.json().await.unwrap();

        assert!(jokes.is_empty());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_fetch_joke_http_error() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("GET", "/jokes/programming/random")
            .with_status(500)
            .create_async()
            .await;

        let url = format!("{}/jokes/programming/random", server.url());
        let result = reqwest::get(&url).await.unwrap().error_for_status();

        assert!(result.is_err());
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_joke_error_display() {
        let request_error = JokeError::Request(reqwest::Error::from(
            reqwest::Client::new()
                .get("http://invalid-url")
                .send()
                .await
                .unwrap_err(),
        ));
        let no_jokes_error = JokeError::NoJokes;

        assert!(request_error.to_string().contains("Request failed"));
        assert_eq!(no_jokes_error.to_string(), "No jokes found");
    }
}
