use cli_clipboard::get_contents;
use std::{env::var, fs, error::Error};
use argh::FromArgs;
use reqwest::{Client, header::AUTHORIZATION};

mod payload;
mod api_response;
mod moa;
use moa::moa;
use payload::Payload;

const URL_V1: &str = "https://api.straico.com/v1";
const COMPLETION: &str = "/prompt/completion";
const MODELS: &str = "/models";

#[derive(FromArgs)]
/// Extra option
struct Cli {
    /// file to attach
    #[argh(option, short ='f')]
    file: Option<String>,
    /// prompt to send
    #[argh(option, short ='p')]
    prompt: Option<String>,
    /// clipboard content
    #[argh(switch, short ='c')]
    clipboard: bool,
    /// if MOA should be used 
    #[argh(switch, short ='m')]
    moa: bool,
    /// list models
    #[argh(switch)]
    models: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let layer_models: [&str; 4] = ["openai/gpt-4o-mini", "mistralai/codestral-mamba", "meta-llama/llama-3.1-405b-instruct", "google/gemini-pro-1.5"];
    let ag_model = "anthropic/claude-3.5-sonnet";

    let api_key: String = match var("STRAICO_API_KEY"){
        Ok(x) => x,
        Err(_) => return Err("No api key found".into())
    };

    let args: Cli = argh::from_env();
    // io::stdin().read_to_string(&mut task)?;
    // if task.trim().is_empty() {
    //     return Err("No input provided".into());
    // }

    let prompt: String = args.prompt.map(|x| wrap(x, "query")).unwrap_or(String::new());
    let file_content = if let Some(file_path) = &args.file {
        wrap(fs::read_to_string(&file_path)?, "file_contents")
    } else { String::new() };
    let clipboard: String = if args.clipboard {
        wrap(get_contents()?, "snippet")
    } else { String::new() };

    let task = format!("{}\n{}\n{}", prompt, clipboard, file_content);

    let client = Client::new();
    let models_url = String::from(URL_V1)+MODELS;
    let comp_url = String::from(URL_V1)+COMPLETION;

    let result: String;
    if task.len() == 2 && args.models { 
        result = client.get(models_url).header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send().await?.text().await?;
    } else if task.len() > 2 && args.moa {
        result = moa(task, &client, &comp_url ,&api_key, layer_models, ag_model).await?;
    } else if task.len() > 2 && !args.moa {
        result = Payload::new(&vec!(ag_model.to_string()), &task)
         .request(&client, &comp_url, &api_key).await?.process_response(ag_model)?;
    } else {
        result = String::from("Nothing to do");
    }

    println!("{}", result);
    Ok(())
}

fn wrap(content: String, wrapper: &str) -> String {
    format!("<{}>\n{}\n</{}>\n", wrapper, content, wrapper)
}
