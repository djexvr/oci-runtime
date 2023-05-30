use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

/// First part of the path to the file managed and created by our program
pub const MAIN_PATH: &str = "./data/";
/// Suffix of the path leading to the status files of the containers
pub const STATUS_SUFF: &str = "status/";

/// States a container may be in
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}


impl Status {
    /// translates from Status object to String.
    pub fn to_string(&self) -> String {
        match self {
            Self::Created => format!("created"),
            Self::Creating => format!("creating"),
            Self::Running => format!("running"),
            Self::Stopped => format!("stopped"),
        }
    }
}

/// Informations contained in a container status file
pub struct State {
    pub id: String,
    pub pid: i64,
    pub status: Status,
    pub bundle: String,
}

/// Reads the status.json file associated with the container, and put these informations in a State struct
pub fn build_status(id: String) -> Result<State,String> {
    let path = format!("{MAIN_PATH}{STATUS_SUFF}{id}.json");
    let content = fs::read_to_string(path.clone()).expect("No container with such ID");
    let value: serde_json::Value = serde_json::from_str(&content[..]).unwrap();

    let pid: i64;
    let status: Status;
    let bundle: String;

    match &value["pid"] {
        Value::Number(n) => {
            pid = n.as_i64().unwrap()
        }
        _ => return Err(format!("Expected PID to be a number in {path}\n", )),
    }
    match &value["bundle"] {
        Value::String(s) => bundle = s.to_string(),
        _ => return Err(format!("Expected Bundle to be a string in {path}\n"))
    }
    match &value["status"] {
        Value::String(s) => {
            match s.as_str() {
                "creating" => status = Status::Creating,
                "created" => status = Status::Created,
                "running" => status = Status::Running,
                "stopped" => status = Status::Stopped,
                _ => return Err(format!("Invalid Status in {path}\n")),
            }
        } 
        _ => return Err(format!("Expected Status to be a string in {path}\n"))
    }

    return Ok(State {
        id,
        pid,
        status,
        bundle,
    })
}

/// Returns a String formated to present the status informations of the container
pub fn state(id: String) -> Result<String,String>{
    let status = build_status(id)?;
    Ok(format!(
        "id: {},\npid: {},\nbundle: {},\nstatus: {}\n",
        status.id, status.pid, status.bundle, status.status.to_string()
    ))
}

/// Changes the state of the container in the status file
pub fn modify_state(id: String, state: Status) -> Result<(),String>{
    let path = format!("{MAIN_PATH}{STATUS_SUFF}{id}.json");
    let content = match fs::read_to_string(path.clone()) {
        Ok(s) => s,
        Err(e) => return Err(format!("Error: No container with such ID:\n{e}\n")),
    };
    let mut value: HashMap<String,Value> = serde_json::from_str(&content[..]).unwrap();
    
    value.insert("status".to_string(), Value::String(state.to_string()));
    let serialized = serde_json::to_string(&value).unwrap();
    let mut f;
    match File::options().write(true).open(path) {
        Ok(file) => f = file,
        Err(e) => return Err(format!("Error: Could not open status file:\n{e}\n")),
    }
    match f.write_all(serialized.trim().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: Unable to write status file:\n{e}\n")),
    }
}
