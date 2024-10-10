//! This module defines the `Payload` struct and its associated methods.
//! 
//! The `Payload` struct represents the data structure for API requests,
//! including models to be used, the message to be processed, and optional
//! parameters like temperature. It also provides functionality to send
//! these payloads as HTTP requests.

use crate::api_response::ApiResponse;
use reqwest::Client;
use serde::Serialize;
use std::error::Error;

/// Represents the payload for an API request.
#[derive(Debug, Serialize, Default)]
pub struct Payload {
    /// List of model names to be used for the request.
    pub models: Vec<String>,
    /// The message to be processed.
    pub message: String,
    /// The temperature parameter for controlling randomness in the response.
    /// Use the default value (0.0) for f64 if 'temperature' is not provided during deserialization.
    #[serde(default)]
    pub temperature: f64,
}

impl Payload {
    /// Creates a new `Payload` instance.
    ///
    /// # Arguments
    ///
    /// * `models` - A vector of model names to be used.
    /// * `message` - The message to be processed.
    /// * `temperature` - An optional temperature value. If None, defaults to 0.0.
    ///
    /// # Returns
    ///
    /// A new `Payload` instance.
    pub fn new(models: Vec<String>, message: String, temperature: Option<f64>) -> Self {
        Self {
            models,
            message,
            temperature: temperature.unwrap_or_default(),
        }
    }

    /// Sends the payload as a request to the specified URL.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client to use for the request.
    /// * `url` - The URL to send the request to.
    /// * `api_key` - The API key for authentication.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `ApiResponse` if successful, or an error if the request fails.
    pub async fn request(
        &self,
        client: &Client,
        url: &str,
        api_key: &str,
    ) -> Result<ApiResponse, Box<dyn Error>> {
        Ok(client
            .post(url)
            .bearer_auth(api_key)
            .json(&self)
            .send()
            .await?
            .json::<ApiResponse>()
            .await?)
    }
}
