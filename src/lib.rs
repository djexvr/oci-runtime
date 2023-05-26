mod parse;
mod state;
mod kill;
mod delete;
mod start;

enum Commands {
    State,
    Create,
    Start,
    Kill,
    Delete,
}
pub struct Config {
    command: Commands,
    id: String,
    path: Option<String>,
    signal: Option<String>,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments: expected at least two arguments");
        }
        let command = args[1].as_str();
        let id: String = args[2].parse().unwrap();
        if command == "state" {
            return Ok(Config {
                command: Commands::State,
                id: id,
                path: None,
                signal: None,
            });
        } else if command == "start" {
            return Ok(Config {
                command: Commands::Start,
                id: id,
                path: None,
                signal: None,
            });
        } else if command == "create" {
            if args.len() < 4 {
                return Err("Expected a third argument: path to bundle");
            } else {
                let path = args[3].to_string();
                return Ok(Config {
                    command: Commands::Create,
                    id: id,
                    path: Some(path),
                    signal: None,
                });
            }
        } else if command == "kill" {
            if args.len() < 4 {
                return Err("Expected a third argument: signal");
            } else {
                let signal: String = args[3].parse().unwrap();
                return Ok(Config {
                    command: Commands::Kill,
                    id: id,
                    path: None,
                    signal: Some(signal),
                });
            }
        } else if command == "delete" {
            return Ok(Config {
                command: Commands::Delete,
                id: id,
                path: None,
                signal: None,
            });
        } else {
            return Err("Unknown command");
        }
    }
}

pub fn run(conf: Config) {
    match conf.command {
        Commands::State => {state::state(conf.id); ()}
        Commands::Create => create(conf.id, conf.path),
        Commands::Start => {start::start(conf.id);()},
        Commands::Kill => {kill::kill(conf.id, conf.signal); ()},
        Commands::Delete => {delete::delete(conf.id);()}
    }
}



fn test_fun(name: &str) {
    use std::process;
    println!("My pid is {}", process::id());
}

mod create;

fn create(id: String, path: Option<String>) {
    use std::path::Path;
    create::create_container_proc(|| {
        test_fun("tony");
        create::init_container_fs(Path::new(
            "/home/djex/Documents/CS/2A/Programmation_systeme/projet_container/alpine",
        )).unwrap();
        0
    });
}
