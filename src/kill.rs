use std::str::FromStr;

use crate::state;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

/// Sends the signal passed as argument to the process running in the container
pub fn kill(id: String, signal: Option<String>) -> Result<(), String> {
    let status = state::build_status(id)?;
    match status.status {
        state::Status::Created | state::Status::Running => {
            let sig: Signal;
            let pid = Pid::from_raw(status.pid.try_into().unwrap());
            match signal {
                None => return Err(format!("Error: No signal provided")),
                Some(s) => match Signal::from_str(&s[..]) {
                    Err(e) => return Err(format!("Error: Invalid signal\n{e}\n")),
                    Ok(s) => sig = s,
                },
            }
            match signal::kill(pid, sig) {
                Err(e) => Err(format!("Error while sending signal:\n{e}\n")),
                Ok(_) => Ok(()),
            }
        }
        _ => Err(format!(
            "Error: Status of container is neither Created nor Running, cannot send signal"
        )),
    }
}
