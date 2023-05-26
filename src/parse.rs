use serde_json::Value;
use std::fs;

pub struct Process {
    pub cwd: String,
    pub env: Vec<String>,
    pub args: Vec<String>,
    pub uid: i64,
    pub gid: i64,
}

pub struct Linux {
    pub namespaces: Vec<String>,
}

pub struct ContainerConfig {
    pub root: String,
    pub process: Process,
    pub linux: Linux
}

pub fn create_config(path: String) -> Result<ContainerConfig,String> {
    let content = fs::read_to_string(path).expect("Unable to read file");
    let value: serde_json::Value = serde_json::from_str(&content[..]).unwrap();

    let root: String;
    let cwd: String;
    let uid: i64;
    let gid: i64;
    let mut env: Vec<String> = Vec::new();
    let mut args: Vec<String> = Vec::new();
    let mut namespaces: Vec<String> = Vec::new();

    match &value["root"] {
        Value::Object(_) => {
            match &value["root"]["path"] {
                Value::String(s) => root = s.clone(),
                Value::Null => return Err(format!("Field root/path should exist in config.json")),
                _ => return Err(format!("Invalid field root/path in config.json")),
            }
        }
        Value::Null=> return Err(format!("Field root should exist in config.json")),
        _ => return Err(format!("Invalid field root in config.json")),
    }

    match &value["process"] {
        Value::Object(_) => {
            match &value["process"]["cwd"] {
                Value::String(s) => cwd = s.clone(),
                Value::Null => return Err(format!("Field process/cwd should exist in config.json")),
                _ => return Err(format!("Invalid field process/cwd in config.json")),
            };
            match &value["process"]["user"] {
                Value::Object(_) => {
                    match &value["process"]["user"]["uid"] {
                        Value::Number(n) => uid = n.as_i64().unwrap(),
                        Value::Null => return Err(format!("Field process/user/uid should exist in config.json")),
                        _ => return Err(format!("Invalie field process/user/uid in config.json")),
                    }
                    match &value["process"]["user"]["gid"] {
                        Value::Number(n) => gid = n.as_i64().unwrap(),
                        Value::Null => return Err(format!("Field process/user/gid should exist in config.json")),
                        _ => return Err(format!("Invalid field process/user/gid in config.json")),
                    }
                },
                Value::Null => return Err(format!("Field process/user should exist in config.json")),
                _ => return Err(format!("Invalid field process/user in config.json")),
            }

            match &value["process"]["env"] {
                Value::Array(vec) => {
                    for val in vec.into_iter() {
                        match val {
                            Value::String(s) => env.push(s.clone()),
                            _ => return Err(format!("Invalid content of array process/env in config.json")),
                        }
                    }
                }
                Value::Null => (),
                _ => return Err(format!("Invalid field process/env in config.json")),
            }

            match &value["process"]["args"] {
                Value::Array(vec) => {
                    for val in vec.into_iter() {
                        match val {
                            Value::String(s) => args.push(s.clone()),
                            _ => return Err(format!("Invalid content of array process/args in config.json")),
                        }
                    }
                }
                Value::Null => (),
                _ => return Err(format!("Invalid field process/args in config.json")),
            }

            }
        Value::Null=> return Err(format!("Field process should exist in config.json")),
        _ => return Err(format!("Invalid field process in config.json")),
    }

    match &value["linux"] {
        Value::Object(_) => {
            match &value["linux"]["namespaces"] {
                Value::Array(vec) => {
                    for val in vec.into_iter() {
                        match val {
                            Value::Object(_) => {
                                match &val["type"] {
                                    Value::String(s) => namespaces.push(s.clone()),
                                    _ => return Err(format!("Invalid field process/linux/namespace/type in config.json"))
                                }
                            },
                            _ => return Err(format!("Invalid content of array process/linux/namespace in config.json")),
                        }
                    }
                },
                _ => return Err(format!("Invalid content of array process/linux/namespace in config.json")),
            }
        },
        _ => (),
    }

    return Ok(ContainerConfig {
        root,
        process: Process {
            uid,
            gid,
            env,
            args,
            cwd,
        },
        linux: Linux {
            namespaces,
        }
    })
}


