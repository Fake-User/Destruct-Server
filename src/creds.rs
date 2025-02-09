use axum::{
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Credentials{
    secret_access_key: String,
    access_key_id: String,
    session_token: String
}

#[derive(Deserialize)]
struct CloudflareResponse{
    result: CloudflareResult,
}

#[derive(Deserialize)]
struct CloudflareResult{
    #[serde(rename = "secretAccessKey")]
    secret_access_key: String,
    #[serde(rename = "accessKeyId")]
    access_key_id: String,
    #[serde(rename = "sessionToken")]
    session_token: String
}

pub async fn creds() -> Result<Json<Credentials>, StatusCode>{
    let client = Client::new();

    let body = serde_json::json!({
        "parentAccessKeyId": env::var("AWS_ACCESS_KEY_ID").unwrap(),
        "permission": "object-read-only",
        "bucket": "destruct-data",
        "ttlSeconds": 3600
    });

    let response = client
        .post(format!("https://api.cloudflare.com/client/v4/accounts/{}/r2/temp-access-credentials", env::var("ACCOUNT_ID").unwrap()))
        .header("X-Auth-Email", env::var("X_AUTH_EMAIL").unwrap())
        .header("X-Auth-Key", env::var("X_AUTH_KEY").unwrap())
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cloudflare_response: CloudflareResponse = response
        .json()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let creds = Credentials{
        secret_access_key: cloudflare_response.result.secret_access_key,
        access_key_id: cloudflare_response.result.access_key_id,
        session_token: cloudflare_response.result.session_token,
    };

    Ok(Json(creds))
}
