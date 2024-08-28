use std::{collections::HashMap, error::Error};
use serde::Deserialize;

#[derive(Debug, Deserialize, Hash)]
enum Models {
    #[serde(rename = "openai/gpt-4o-mini")]
    OpenaiGpt4oMini,
    #[serde(rename = "mistralai/codestral-mamba")]
    MistralaiCodestralMamba,
    #[serde(rename = "meta-llama/llama-3.1-405b-instruct")]
    MetaLlama3_1_405bInstruct,
    #[serde(rename = "google/gemini-pro-1.5")]
    GoogleGeminiPro1_5,
    #[serde(rename = "anthropic/claude-3.5-sonnet")]
    AnthropicClaude3_5Sonnet
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse { data: Data, } // success: bool,

#[derive(Debug, Deserialize)]
pub struct Data { completions: HashMap<String, Model>, } 
// overall_price: Price, overall_words: Words,

#[derive(Debug, Deserialize)]
pub struct Model { completion: Completion, }
// price: Price, words: Words,

#[derive(Debug, Deserialize)]
pub struct Completion { choices: Vec<Choice>, }
// id:  String, model: String, object: String, created: usize, usage: Usage,

#[derive(Debug, Deserialize)]
pub struct Choice { message: Message, }
// index: u8, finish_reason: String,

#[derive(Debug, Deserialize)]
pub struct Message { content: String, }
// role: String,

impl ApiResponse {
    pub fn process_response(&self, model: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.data.completions.get(model).map(|m| m.completion.choices[0].message
            .content.to_owned()).ok_or("Model not found in response")?)
    }

    pub fn process_middle(&self, models: [&str; 4], ag_prompt: &String) -> 
        Result<String, Box<dyn Error>> {
        let mut output = ag_prompt.clone();
        for (i, model) in models.iter().enumerate() {
            let model_response = self.process_response(model)?;
            output.push_str(&format!(
                "<model{}_response>\n{}\n</model{}_response>\n\n",i+1,model_response,i+1));
            }
        Ok(output)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_deserialization() {
        // Construct a sample JSON object that adheres to the ApiResponse structure
        let json_data = json!({
            "data": {
                "completions": {
                    "openai/gpt-4o-mini": {
                        "completion": {
                            "choices": [
                                {
                                    "message": {
                                        "content": "This is a response from the OpenAI model."
                                    }
                                }
                            ]
                        }
                    },
                    "meta-llama/llama-3.1-405b-instruct": {
                        "completion": {
                            "choices": [
                                {
                                    "message": {
                                        "content": "This is a response from the Meta Llama model."
                                    }
                                }
                            ]
                        }
                    }
                }
            }
        });

        let bad_data = json!({"bad data": "not like this"});

        // Attempt to deserialize the JSON data into ApiResponse
        let result: Result<ApiResponse, _> = serde_json::from_value(json_data);
        let bad_result: Result<ApiResponse, _> = serde_json::from_value(bad_data);

        // Assert that deserialization was successful
        assert!(result.is_ok(), "Deserialization failed: {:?}", result.err());
        assert!(bad_result.is_err(), "Deserialization should have failed");
        }
}
