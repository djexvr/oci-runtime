mod parse;
mod state;

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
    signal: Option<i32>,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config,&str> {
        if args.len()<3 {
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
            })
        } else if command == "start" {
            return Ok(Config {
                command: Commands::Start,
                id: id,
                path: None,
                signal: None,

            })
        } else if command == "create" {
            if args.len()<4 {
                return Err("Expected a third argument: path to bundle");
            } else {
                let path = args[3].to_string();
                return Ok(Config {
                    command: Commands::Create,
                    id: id,
                    path: Some(path),
                    signal: None,
                })
            } 
        } else if command == "kill" {
            if args.len()<4 {
                return Err("Expected a third argument: signal");
            } else {
                let signal: i32 = args[3].parse().unwrap();
                return Ok(Config {
                    command: Commands::Kill,
                    id: id,
                    path: None,
                    signal: Some(signal),
                })
            } 
        } else if command == "delete" {
            return Ok(Config {
                command: Commands::Delete,
                id: id,
                path: None,
                signal: None,
            })
        }
        else {
            return Err("Unknown command");
        }
    }
}

pub fn run(conf: Config) {
    match conf.command {
        Commands::State => {state::state(conf.id); ()}
        Commands::Create => create(conf.id,conf.path),
        Commands::Start => start(conf.id),
        Commands::Kill => kill(conf.id, conf.signal),
        Commands::Delete => delete(conf.id),
    }
}


fn create(id: String,path: Option<String>) {

}

fn start(id: String) {

}

fn kill(id: String, signal: Option<i32>) {

}

fn delete(id: String) {

}
