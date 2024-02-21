use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use reqwest::{Client, Error, Response};
use serde_json::{from_str, Value};

#[derive(Parser)]
struct ClapArgs {
    #[clap(subcommand)]
    pub command: Args,
}
struct Command;

#[derive(Subcommand)]
enum Args {
    /// Get the currently time
    Time,

    /// Translate senteces: 'etos' -> english to spanish.
    Etos,

    /// Traduce oraciones: 'stoe' -> spanish to english.
    Stoe,
}

#[tokio::main]
async fn main() {
    let arguments = ClapArgs::parse();

    match arguments.command {
        Args::Time => Command::get_time(),
        Args::Etos => Command::translations("en|es", "food")
            .await
            .expect("poronga de indio"),
        Args::Stoe => Command::translations("es|en", "amarillo")
            .await
            .expect("poronga de indio"),
    }
}

impl Command {
    fn get_time() {
        let local_time: DateTime<Local> = Local::now();
        println!("{}", local_time.format("%H:%M:%S - %d/%m/%y"));
    }

    async fn translations(language: &str, setence: &str) -> Result<(), Error> {
        const URL: &'static str = "https://api.mymemory.translated.net/get";

        let params: [(&str, &str); 2] = [("q", setence), ("langpair", language)];

        let client: Client = Client::new();
        let response: Response = match client.post(URL).form(&params).send().await {
            Ok(res) => res,
            Err(error) => {
                eprintln!("lym: error sending translation request: {error}");
                return Ok(());
            }
        };

        if response.status().is_success() {
            let body_text: String = match response.text().await {
                Ok(text) => text,
                Err(error) => {
                    eprintln!("lym: error retrieving translation response body: {error}");
                    return Ok(());
                }
            };

            let body: Value = match from_str(&body_text) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("lym: error converting response to JSON: {error}");
                    return Ok(());
                }
            };

            if let Some(translated_word) = body["responseData"]["translatedText"].as_str() {
                println!("translation -> {}", translated_word);
            }
        } else {
            eprintln!(
                "lym: error translation request failed with status {}",
                response.status()
            );
        }
        Ok(())
    }
}
