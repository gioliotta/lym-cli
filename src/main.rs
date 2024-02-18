use chrono::{DateTime, Local};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "lym", about = "Command line for programers")]
struct Cli {
    #[structopt(short = "h", long = "help")]
    help: bool,
    #[structopt(short = "v", long = "version")]
    version: bool,
    #[structopt(long = "time")]
    time: bool,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::from_args();
    let local_time: DateTime<Local> = Local::now();

    if args.help {
        Command::get_help();
    } else if args.version {
        Command::get_version();
    } else if args.time {
        Command::get_time(local_time)
    }
}

struct Command;

impl Command {
    fn get_help() {
        print!(
            "
Commands:
    lym               Information and commands available from lym.
    lym --help        Only available commands.
    lym --version     Current version.
    lym time          Check current date and hour.
    lym hour          Check current hour.
    lym date          Check current date.
    lym etos <word>   Translate a word or sentence from English to Spanish.
    lym stoe <word>   Translate a word or sentence from Spanish to English.
    lym todos         View todolist.
    lym add <text>    Add a new todo to todolist.
    lym rm <index>    Remove a task by index.
    lym exit          Close lym cli.

"
        );
    }

    fn get_version() {
        println!("lym version 1.0.0");
    }

    fn get_time(local_time: DateTime<Local>) {
        println!("{}", local_time.format("%H:%M:%S - %d/%m/%y"));
    }

    /* //TODO: implementar 'todos' con una db

    let mut todos_vec: Vec<String> = Vec::new();

    #[structopt(long = "todos")]
    todos: bool,

    #[structopt(long = "add")]
    add_todo: Vec<String>,

    else if args.todos {
        Command::get_todos(&mut todos_vec)
    } else if !args.add_todo.is_empty() {
        let new_todo: String = args.add_todo.join(" ");
        Command::add_todo(&mut todos_vec, new_todo);
    }

    fn get_todos(todos: &Vec<String>) {
        match todos.len() {
            0 => println!("Add your first task!"),
            _ => {
                for (index, td) in todos.iter().enumerate() {
                    println!("  {} - {}", index + 1, td);
                }
            }
        }
    }

    fn add_todo(todos: &mut Vec<String>, new_todo: String) {
        todos.push(new_todo);
        println!("New todo added successfully!");
    }

    fn remove_todo(todos: &mut Vec<String>, index_td: String) {
        let index_td: usize = index_td[3..].parse().unwrap();
        let index_td: usize = index_td - 1;

        if index_td < todos.len() {
            todos.remove(index_td);
            println!("Tarea eliminada correctamente.");
        } else {
            println!("Índice fuera de rango. No se eliminó ninguna tarea.");
        }
    } */
}

/* Código viejo.

use chrono::{DateTime, Local};
use colored::Colorize;
use reqwest::{Client, Error, Response};
use serde_json::{from_str, Value};
use std::fs::{read_to_string, File};
use std::io::{stdin, ErrorKind};


#[tokio::main]
async fn main() {
    let mut todos_vec: Vec<String> = Vec::new();

    loop {
        let mut command: String = String::new();

        stdin()
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
        let cmd_string: String = String::from(command);

        let result: Result<(), Error> = match command {
            "" => ,
            "--help" | "-h" => Command::get_help(),
            "--version" | "-v" => Command::get_version(),
            "todos" => Command::get_todos(&todos_vec),
            "time" => Command::get_time(local),
            "hour" => Command::get_hour(local),
            "date" => Command::get_date(local),
            "open" => Command::open_file(),
            "read" => Command::read_file(),
            cmd if cmd.contains("add") => Command::add_todo(&mut todos_vec, cmd_string),
            cmd if cmd.contains("rm") => Command::remove_todo(&mut todos_vec, cmd_string),
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
    fn open_file() -> Result<(), Error>  {
        File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });

Ok(())
    }

    fn read_file() -> Result<(), Error>  {
        let loco: Result<String, std::io::Error> = read_to_string("hello.txt");
        println!("{:?}", loco);

    }

    fn get_lym() -> Result<(), Error>  {
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
    lym etos <text>   Translate a word or sentence from English to Spanish.
    lym stoe <text>   Translate a word or sentence from Spanish to English.
    lym todos         View todolist.
    lym add <text>    Add a new todo to todolist.
    lym rm <index>    Remove a task by index.
    lym exit          Close lym cli.

    ",
            "lym version 1.0.0 - command line for programmers".magenta(),
            "https://github.com/gioliotta".white(),
            "Commands".white(),
        );
Ok(())
    }

    fn get_help() -> Result<(), Error>  {
        print!(
            "
{}:
    lym               Information and commands available from lym.
    lym --help        Only available commands.
    lym --version     Current version.
    lym time          Check current date and hour.
    lym hour          Check current hour.
    lym date          Check current date.
    lym etos <word>   Translate a word or sentence from English to Spanish.
    lym stoe <word>   Translate a word or sentence from Spanish to English.
    lym todos         View todolist.
    lym add <text>    Add a new todo to todolist.
    lym rm <index>    Remove a task by index.
    lym exit          Close lym cli.

",
            "Commands".white(),
        );
Ok(())
    }

    fn get_version() -> Result<(), Error>  {
        println!("lym version 1.0.0");
Ok(())
    }

    fn get_time(local_time: DateTime<Local>) -> Result<(), Error>   {
        println!("{}", local_time.format("%H:%M:%S / %d-%m-%y"));
Ok(())
    }

    fn get_hour(local_hour: DateTime<Local>) -> Result<(), Error>  {
        println!("{}", local_hour.format("%H:%M:%S"));
Ok(())
    }

    fn get_date(local_date: DateTime<Local>) -> Result<(), Error>  {
        println!("{}", local_date.format("%d-%m-%y"));
Ok(())
    }

    async fn translations(command: &str, language: Languages) -> Result<(), Error>  {
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
                return ;
            }
        };

        if response.status().is_success() {
            let body_text: String = match response.text().await {
                Ok(text) => text,
                Err(error) => {
                    eprintln!("lym: error retrieving translation response body: {error}");
                    return ;
                }
            };

            let body: Value = match from_str(&body_text) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("lym: error converting response to JSON: {error}");
                    return ;
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

    fn add_todo(todos: &mut Vec<String>, command: String) -> Result<(), Error>  {
        let td: &str = &command[4..];
        todos.push(td.to_string());
Ok(())
    }

    fn remove_todo(todos: &mut Vec<String>, index_td: String) -> Result<(), Error>  {
        let index_td: usize = index_td[3..].parse().unwrap();
        let index_td: usize = index_td - 1;

        if index_td < todos.len() {
            todos.remove(index_td);
            println!("Tarea eliminada correctamente.");
        } else {
            println!("Índice fuera de rango. No se eliminó ninguna tarea.");
        }
Ok(())
    }

    fn get_todos(todos: &Vec<String>) -> Result<(), Error>  {
        match todos.len() {
            0 => println!("Add your first task!"),
            _ => {
                for (index, td) in todos.iter().enumerate() {
                    println!("  {} - {}", index + 1, td);
                }
            }
        }
Ok(())
    }

    fn invalid_command(invalid_cmd: &str) -> Result<(), Error>  {
        eprintln!("lym: '{invalid_cmd}' is not a lym command. See 'lym --help'.");
Ok(())
    }
}

enum Languages {
    EN(String),
    ES(String),
}
*/
