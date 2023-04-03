use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use clap::Parser;
use colored::Colorize;
use std::error::Error;

use spinners::{Spinner, Spinners};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "You are a helpful assistant. You are helping a software developer with a programming question. The programmer asks you a question. You reply:"
    )]
    prompt: String,
    #[arg(short, long)]
    message: String,
    #[arg(short, long, default_value = "1")]
    number_of_choices: u8,
    #[arg(short, long, default_value = "0.6")]
    temperature: f32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(args.prompt)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(args.message)
                .build()?,
        ])
        .temperature(args.temperature)
        .top_p(0.6)
        .n(args.number_of_choices)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .build()?;

    let mut spinner = Spinner::new(Spinners::Dots12, "Thinking...".to_string());

    let chat = client.chat();
    let response = chat.create(request).await?;

    spinner.stop();
    for choice in response.choices {
        println!("-----------------------------------");
        println!(
            "{}\n{}:\n{}\n",
            choice.index,
            "Answer".bold(),
            choice.message.content.green()
        );
    }

    Ok(())
}
