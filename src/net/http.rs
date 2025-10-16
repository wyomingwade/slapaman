// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Wyoming Wade

use reqwest::{Client, Response};

pub async fn get_request(url: &String) -> Result<Response, String> {
    // a GET request wrapper that injects the user agent header
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "slapaman/0.1.0 (GitHub: @wyomingwade)")
        .send()
        .await
        .map_err(|e| format!("failed to send GET request: {}", e))?;

    Ok(response)
}