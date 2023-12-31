mod create;
mod delete;
mod kill;
mod parse;
mod start;
mod state;

/// The five commands that may be called through the CLI
enum Commands {
    State,
    Create,
    Start,
    Kill,
    Delete,
}

/// Informations that may be contained in the arguments passed through the CLI.
/// Path must only be present if the command is create.
/// Signal must only be present if the command is kill.
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

/// Calls the relevant fonction, depending on the command passed, and manages the results.
pub fn run(conf: Config) {
    let res = match conf.command {
        Commands::State => match state::state(conf.id) {
            Ok(s) => {
                println!("{}", s);
                Ok(())
            }
            Err(s) => Err(s),
        },
        Commands::Create => create::create(conf.id, conf.path.unwrap()),
        Commands::Start => start::start(conf.id),
        Commands::Kill => kill::kill(conf.id, conf.signal),
        Commands::Delete => delete::delete(conf.id),
    };
    match res {
        Err(s) => println!("{}", s),
        _ => (),
    }
}
