use std::str::FromStr;

use super::state;
use nix::unistd::Pid;
use nix::sys::signal::{self, Signal};

pub fn kill(id: String, signal: Option<String>) -> Result<(),String> {
    let status =  state::build_status(id)?;
    match status.status {
        state::Status::Created |
        state::Status:: Running => {
            let sig: Signal;
            let pid = Pid::from_raw(status.pid.try_into().unwrap());
            match signal {
                None => return Err(format!("Error: No signal provided")),
                Some(s) => {
                    match Signal::from_str(&s[..]) {
                        Err(_) => return Err(format!("Error: Invalid signal")),
                        Ok(s) => sig = s,
                    }
                }
            }
            match signal::kill(pid, sig) {
                Err(_) => Err(format!("Error while sending signal")),
                Ok(_) => Ok(()),
            }
        },
        _ => Err(format!("Error: Status of container is neither Created nor Running, cannot send signal")),
    }
}