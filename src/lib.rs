use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Local,
};
use clap::{error::Result, Parser, Subcommand};
use eval::eval;
use rand::Rng;
use reqwest::{Client, Error, Response};
use serde_json::{from_str, Value};

#[derive(Parser)]
pub struct ClapArgs {
    #[clap(subcommand)]
    command: Args,
}

#[derive(Subcommand)]
enum Args {
    /// Obtén la versión actual de lym.
    Version,

    /// Obtén la fecha y hora actual.
    Time,

    /// Traduce texto de inglés al español.
    En { text: Vec<String> },

    /// Traduce texto de español al inglés.
    Es { text: Vec<String> },

    /// Realiza operaciones matemáticas.
    Cal { operation: String },

    /// Genera una contraseña aleatoria.
    #[clap(value_parser, allow_hyphen_values = true)]
    Pass { length: Option<String> },
}

pub async fn run(args: ClapArgs) {
    const EN_TO_ES: &str = "en|es";
    const ES_TO_EN: &str = "es|en";

    match args.command {
        Args::Version => Command::get_version(),
        Args::Time => Command::get_time(),
        Args::Cal { operation } => Command::calculate(&operation),
        Args::En { text } => Command::translate(EN_TO_ES, &text)
            .await
            .expect("Error en el comando 'en'"),
        Args::Es { text } => Command::translate(ES_TO_EN, &text)
            .await
            .expect("Error en el comando 'es'"),
        Args::Pass { length } => Command::generate_password(length),
    }
}

struct Command;

impl Command {
    fn get_version() {
        println!("versión de lym: 1.1.0");
    }

    fn get_time() {
        let local_time: DateTime<Local> = Local::now();
        let date: DelayedFormat<StrftimeItems> = local_time.format("%H:%M:%S - %d/%m/%y");
        println!("{}", date);
    }

    fn calculate(operation: &String) {
        if operation.trim().len() <= 1 {
            panic!("Operación inválida");
        }

        match eval(operation) {
            Ok(result) => println!("{result}"),
            Err(error) => println!("{error}"),
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

    fn generate_password(length: Option<String>) {
        const DEFAULT_VAL: i32 = 8;
        const MAX: i32 = 10000;
        const CHARSET: &[u8; 69] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
            abcdefghijklmnopqrstuvwxyz\
            0123456789\
            !@#$*_.";

        let length_pass: i32 = match length {
            Some(len) => match len.parse::<i32>() {
                Ok(len) if len > 0 && len < MAX => len,
                _ => return eprintln!("Ingresa un número entero positivo y/o menor a {MAX}"),
            },
            None => DEFAULT_VAL,
        };

        let mut rng = rand::thread_rng();
        let password: String = (0..length_pass)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        println!("{}", password);
    }
}
