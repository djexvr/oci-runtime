use super::state;
use state::build_status;
use std::fs::remove_file;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};

/// Function used to start the desired command inside the container host side.
/// Sends start message and receives a response, then updates container state to running in status file
pub fn start(id: String) -> Result<(), String> {
    let status = state::build_status(id.clone())?;
    match status.status {
        state::Status::Created => {
            send_start(id.clone())?;

            state::modify_state(id, state::Status::Running)
        }
        _ => Err(format!(
            "Error: Status of container is not Created, cannot start"
        )),
    }
}

/// Sends a start message to the container via a socket
fn send_start(id: String) -> Result<(), String> {
    let status = build_status(id.clone())?;
    let socket_path = format!("{}/start", status.bundle);

    let mut input_stream = match UnixStream::connect(socket_path.as_str()) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!(
                "Error: Could not connect to input socket host side:\n{e}\n"
            ))
        }
    };
    match input_stream.write(b"Start") {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(format!(
                "Error: Failed to write on stream host side:\n{e}\n"
            ))
        }
    }
}

/// Listens for a "Start" message from the host on a socket
pub fn receive_start() -> Result<(), String> {
    let socket_path: &str = "start";

    let unix_listener = match UnixListener::bind(socket_path) {
        Ok(s) => s,
        Err(e) => return Err(format!("Error: could not create socket:\n{e}\n")),
    };

    let (mut input_stream, _socket_address) = match unix_listener.accept() {
        Ok(s) => s,
        Err(e) => {
            return Err(format!(
                "Error: Problem while accepting start socket connection:\n{e}\n"
            ))
        }
    };

    let mut message = String::new();
    match input_stream.read_to_string(&mut message) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "Error: Could not get response container side:\n{e}\n"
            ))
        }
    }
    match message.as_str() {
        "Start" => (),
        _ => return Err(format!("Error: Could not start process container side")),
    }

    match remove_file(socket_path) {
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}
