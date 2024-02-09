use chrono::{DateTime, Local};
use colored::Colorize;
use reqwest::{Client, Error, Response};
use serde_json::{from_str, Value};
use std::io;

#[tokio::main]
async fn main() {
    loop {
        let mut command: String = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("lym: error reading command");

        let command: &str = command.trim();

        if command == "lym exit" || command == "exit" {
            break;
        }

        if command == "lym" {
            Command::get_lym().expect("lym: error with 'lym' command");
            continue;
        }

        let command: &str = &command[4..];

        let english_to_spanish: Languages = Languages::EN(String::from("en|es"));
        let spanish_to_english: Languages = Languages::ES(String::from("es|en"));
        let local: DateTime<Local> = Local::now();

        let result: Result<(), Error> = match command {
            "" => Ok(()),
            "--help" | "-h" => Command::get_help(),
            "--version" | "-v" => Command::get_version(),
            "time" => Command::get_time(local),
            "hour" => Command::get_hour(local),
            "date" => Command::get_date(local),
            cmd if cmd.contains("etos") => Command::translations(command, english_to_spanish).await,
            cmd if cmd.contains("stoe") => Command::translations(command, spanish_to_english).await,
            _ => Command::invalid_command(command),
        };

        if let Err(error) = result {
            eprintln!("lym: error in command execution: {error}");
            continue;
        }
    }
}

struct Command;

impl Command {
    fn get_lym() -> Result<(), Error> {
        print!(
            "{}
for contributions -> {}

 {}:
    lym               Information and commands available from lym.
    lym --help        Only available commands.
    lym --version     Current version.
    lym time          Check current date and hour.
    lym hour          Check current hour.
    lym date          Check current date.
    lym etos <word>   Translate a word from English to Spanish.
    lym stoe <word>   Translate a word from Spanish to English.
    lym exit          Close lym cli.

    ",
            "lym version 1.0.0 - command line for programmers".magenta(),
            "https://github.com/gioliotta".white(),
            "Commands".white(),
        );
        Ok(())
    }

    fn get_help() -> Result<(), Error> {
        print!(
            "  
{}:
    lym               Information and commands available from lym.
    lym --help        Only available commands.
    lym --version     Current version.
    lym time          Check current date and hour.
    lym hour          Check current hour.
    lym date          Check current date.
    lym etos <word>   Translate a word from English to Spanish.
    lym stoe <word>   Translate a word from Spanish to English.
    lym exit          Close lym cli.

",
            "Commands".white(),
        );
        Ok(())
    }

    fn get_version() -> Result<(), Error> {
        println!("lym version 1.0.0");
        Ok(())
    }

    fn get_time(local_time: DateTime<Local>) -> Result<(), Error> {
        println!("{}", local_time.format("%H:%M:%S / %d-%m-%y"));
        Ok(())
    }

    fn get_hour(local_hour: DateTime<Local>) -> Result<(), Error> {
        println!("{}", local_hour.format("%H:%M:%S"));
        Ok(())
    }

    fn get_date(local_date: DateTime<Local>) -> Result<(), Error> {
        println!("{}", local_date.format("%d-%m-%y"));
        Ok(())
    }

    async fn translations(command: &str, language: Languages) -> Result<(), Error> {
        let collect_words: Vec<&str> = command.split_whitespace().collect();
        let remove_cmd: Vec<&str> = collect_words.into_iter().skip(1).collect();

        let sentence_string: String = remove_cmd.join(" ");
        let sentence_to_translate: &str = sentence_string.as_str();

        let lan_as_str: &str = match language {
            Languages::EN(_) => "en|es",
            Languages::ES(_) => "es|en",
        };

        const URL: &'static str = "https://api.mymemory.translated.net/get";
        let params: [(&str, &str); 2] = [("q", sentence_to_translate), ("langpair", lan_as_str)];

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
                println!("translation -> {translated_word}");
            }
        } else {
            eprintln!(
                "lym: error translation request failed with status {}",
                response.status()
            );
        }

        Ok(())
    }

    fn invalid_command(invalid_cmd: &str) -> Result<(), Error> {
        eprintln!("lym: '{invalid_cmd}' is not a lym command. See 'lym --help'.");
        Ok(())
    }
}

enum Languages {
    EN(String),
    ES(String),
}
