use std:: error::Error;
use reqwest::{Client, header::AUTHORIZATION};
use serde::Serialize;

use crate::api_response::ApiResponse;

#[derive(Debug, Serialize)]
pub struct Payload { models: Vec<String>, message: String }

impl Payload {
    pub fn new(models: &Vec<String>, message: &String) -> Self {
        Self { models: models.to_owned(), message: message.to_owned() }
    }

    pub async fn request(&self, client: &Client, url: &str, api_key: &String) -> Result<ApiResponse, Box<dyn Error>> {
        Ok(client.post(url)
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .json(&self).send().await?.json::<ApiResponse>().await?)
        }
}
