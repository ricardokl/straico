use clap::{Parser, ArgGroup};
use cli_clipboard::get_contents;
use reqwest::{header::AUTHORIZATION, Client};
use std::{env::var, error::Error, fs};

mod api_response;
mod moa;
mod payload;
use moa::moa;
use payload::Payload;

const URL_V1: &str = "https://api.straico.com/v1";
const COMPLETION: &str = "/prompt/completion";
const MODELS: &str = "/models";

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(group(
    ArgGroup::new("base_flags")
        .required(true)
        .args(&["prompt", "models", "user"])
))]
struct Cli {
    /// Prompt to send
    #[arg(short, long)]
    prompt: Option<String>,

    /// List models (must be used alone)
    #[arg(long)]
    models: bool,

    /// User flag (must be used alone)
    #[arg(long)]
    user: bool,

    /// Files to attach (requires '--prompt')
    #[arg(short, long, requires = "prompt")]
    files: Vec<String>,

    /// Use clipboard content (requires '--prompt')
    #[arg(short, long, requires = "prompt")]
    clipboard: bool,

    /// Use MOA (requires '--prompt')
    #[arg(short, long, requires = "prompt")]
    moa: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let layer_models: [&str; 4] = [
        "openai/gpt-4o-mini",
        "mistralai/codestral-mamba",
        "meta-llama/llama-3.1-405b-instruct",
        "google/gemini-pro-1.5",
    ];
    let ag_model = "anthropic/claude-3.5-sonnet";

    let api_key: String = match var("STRAICO_API_KEY") {
        Ok(x) => x,
        Err(_) => return Err("No api key found".into()),
    };

    let args = Cli::parse();

    // The argument validation is now handled by clap

    let client = Client::new();
    let models_url = format!("{}{}", URL_V1, MODELS);
    let comp_url = format!("{}{}", URL_V1, COMPLETION);

    if args.models {
        let result = client
            .get(models_url)
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await?
            .text()
            .await?;
        println!("{}", result);
        return Ok(());
    }

    if args.user {
        // Handle 'user' flag action here
        println!("User flag used.");
        return Ok(());
    }

    // Proceed with prompt handling
    let prompt = wrap(args.prompt.unwrap_or_default(), "query");

    let mut file_contents = String::new();
    for file_path in &args.files {
        let content = fs::read_to_string(file_path)?;
        file_contents.push_str(&wrap(content, "file_contents"));
    }

    let clipboard_content = if args.clipboard {
        wrap(get_contents()?, "snippet")
    } else {
        String::new()
    };

    let task = format!("{}\n{}\n{}", prompt, clipboard_content, file_contents);

    let result = if args.moa {
        moa(task, &client, &comp_url, &api_key, layer_models, ag_model).await?
    } else {
        Payload::new(&vec![ag_model.to_string()], &task)
            .request(&client, &comp_url, &api_key)
            .await?
            .process_response(ag_model)?
    };

    println!("{}", result);
    Ok(())
}

fn wrap(content: String, wrapper: &str) -> String {
    format!("<{}>\n{}\n</{}>\n", wrapper, content, wrapper)
}
