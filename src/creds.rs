use axum::{
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
}

#[derive(Deserialize)]
struct CloudflareResponse {
    result: CloudflareResult,
}

#[derive(Deserialize)]
struct CloudflareResult {
    #[serde(rename = "accessKeyId")]
    access_key_id: String,
    #[serde(rename = "secretAccessKey")]
    secret_access_key: String,
    #[serde(rename = "sessionToken")]
    session_token: String,
}

pub async fn get_creds() -> Result<Json<Credentials>, StatusCode> {
    let client = Client::new();

    let body = serde_json::json!({
        "bucket": "destruct-data",
        "parentAccessKeyId": env::var("AWS_ACCESS_KEY_ID").unwrap(),
        "permission": "object-read-only",
        "ttlSeconds": 3600
    });

    let response = client
        .post(format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/r2/temp-access-credentials",
            env::var("ACCOUNT_ID").unwrap()
        ))
        .header("Content-Type", "application/json")
        .header("X-Auth-Email", env::var("X_AUTH_EMAIL").unwrap())
        .header("X-Auth-Key", env::var("X_AUTH_KEY").unwrap())
        .json(&body)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cloudflare_response: CloudflareResponse = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let creds = Credentials {
        access_key_id: cloudflare_response.result.access_key_id,
        secret_access_key: cloudflare_response.result.secret_access_key,
        session_token: cloudflare_response.result.session_token,
    };

    Ok(Json(creds))
}
