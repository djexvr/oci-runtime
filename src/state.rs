use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;

pub const MAIN_PATH: &str = "~/.oci-runtime/";
pub const STATUS_SUFF: &str = "container_statuses/";
pub const FOLDER_SUFF: &str = "container_folders/";

pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Self::Created => format!("Created"),
            Self::Creating => format!("Creating"),
            Self::Running => format!("Running"),
            Self::Stopped => format!("Stopped"),
        }
    }
}
pub struct State {
    pub id: String,
    pub pid: i64,
    pub status: Status,
    pub bundle: String,
}

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
        _ => return Err(format!("Expected PID to be a number in {path}", )),
    }
    match &value["bundle"] {
        Value::String(s) => bundle = s.to_string(),
        _ => return Err(format!("Expected Bundle to be a string in {path}"))
    }
    match &value["status"] {
        Value::String(s) => {
            let st = s.to_string();
            if st == format!("creating") {
                status = Status::Creating
            } else if st == format!("created") {
                status = Status::Created
            } else if st == format!("Running") {
                status = Status::Running
            } else if st == format!("Stopped") {
                status = Status::Stopped
            } else {
                return Err(format!("Invalid Status in {path}"))
            }
        } 
        _ => return Err(format!("Expected Status to be a string in {path}"))
    }

    return Ok(State {
        id,
        pid,
        status,
        bundle,
    })
}

pub fn state(id: String) -> Result<String,String>{
    let status = build_status(id)?;
    Ok(format!(
        "id: {},\n pid: {},\n bundle: {},\n status: {}",
        status.id, status.pid, status.bundle, status.status.to_string()
    ))
}

pub fn modify_state(id: String, state: Status) -> Result<(),String>{
    let path = format!("{MAIN_PATH}{STATUS_SUFF}{id}.json");
    let content = match fs::read_to_string(path.clone()) {
        Ok(s) => s,
        Err(_) => return Err(format!("Error: No container with such ID")),
    };
    let mut value: serde_json::Value = serde_json::from_str(&content[..]).unwrap();
    
    let status = match state {
        Status::Created => format!("created"),
        Status::Creating => format!("creating"),
        Status::Running => format!("running"),
        Status::Stopped => format!("stopped"),
    };
    value["status"] = Value::String(status);
    let serialized = serde_json::to_string(&value).unwrap();
    let mut f = File::open(path).expect("Unable to open file");
    match f.write_all(serialized.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Error: Unable to write status file")),
    }
}
