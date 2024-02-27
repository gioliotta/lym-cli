use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use eval::eval;
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
    /// Obtén la version actual de lym.
    Version,

    /// Obtén la fecha y hora actual.
    Time,

    /// Traduce texto  de inglés a español.
    Etos { text: Vec<String> },

    /// Traduce texto  de español a inglés.
    Stoe { text: Vec<String> },

    /// Realiza operaciones matemáticas.
    Cal { operation: String },
}

#[tokio::main]
async fn main() {
    let arguments: ClapArgs = ClapArgs::parse();

    match arguments.command {
        Args::Version => Command::get_version(),
        Args::Time => Command::get_time(),
        Args::Cal { operation } => Command::calculate(&operation),
        Args::Etos { text } => Command::translate("en|es", &text)
            .await
            .expect("Error in etos command"),
        Args::Stoe { text } => Command::translate("es|en", &text)
            .await
            .expect("Error in stoe command"),
    }
}

impl Command {
    fn get_version() {
        println!("lym version 1.0.0")
    }

    fn get_time() {
        let local_time: DateTime<Local> = Local::now();
        println!("{}", local_time.format("%H:%M:%S - %d/%m/%y"));
    }

    fn calculate(operation: &String) {
        match eval(operation) {
            Ok(val) => println!("{}", val),
            Err(err) => println!("Error on calculate: {}", err),
        }
    }

    async fn translate(language: &str, sentence: &Vec<String>) -> Result<(), Error> {
        const URL: &'static str = "https://api.mymemory.translated.net/get";
        let sentence: String = sentence.join(" ");
        let sentence: &str = sentence.as_str();

        let params: [(&str, &str); 2] = [("q", sentence), ("langpair", language)];

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
                println!("{}", translated_word);
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

// sudo cp /home/gixi/toding/programasao/lym-cli/target/release/lym /usr/local/bin
