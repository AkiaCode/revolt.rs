#[derive(Clone)]
pub struct Http;


impl Http {
    pub async fn post(url: &str, json: &serde_json::Value, token: &str) -> Result<reqwest::Response, std::string::String> {
            let resp = reqwest::Client::new()
            .post(url)
            .json(json)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::USER_AGENT, "revolt.rs v0.1")
            .header("x-bot-token", token)
            .send()
            .await
            .map_err(|e| e.to_string());
        resp
    }
    pub async fn get(url: &str, token: &str) -> Result<reqwest::Response, std::string::String> {
        let resp = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::USER_AGENT, "revolt.rs v0.1")
        .header("x-bot-token", token)
        .send()
        .await
        .map_err(|e| e.to_string());
    resp
}
}