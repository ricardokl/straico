use reqwest::Client;
use std::error::Error;

use crate::payload::Payload;

pub async fn moa(
    task: String,
    client: &Client,
    url: &str,
    api_key: &String,
    layer_models: [&str; 4],
    ag_model: &str,
) -> Result<String, Box<dyn Error>> {
    let mut current_prompt: String = task.to_owned();
    let ag_prompt: String = include_str!(".././ag_prompt.txt").replace("{}", &task);

    let models: Vec<String> = layer_models.iter().map(|x| x.to_string()).collect();
    for _ in 1..=4u8 {
        current_prompt = Payload::new(&models, &current_prompt, None)
            .request(&client, url, &api_key)
            .await?
            .process_middle(layer_models, &ag_prompt)?;
    }

    let models: Vec<String> = vec![ag_model.to_string()];
    current_prompt = Payload::new(&models, &current_prompt, None)
        .request(&client, url, &api_key)
        .await?
        .process_response(ag_model)?;

    Ok(current_prompt.to_string())
}
