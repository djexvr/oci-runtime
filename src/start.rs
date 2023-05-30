use super::state;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};


/// Function used to start the desired inside the container host side.
/// Sends start message and receives a response, then updates container state to running in status file
pub fn start(id: String) -> Result<(),String> {
    let status =state::build_status(id.clone())?;
    match status.status {
        state::Status::Created => {
            send_start(id.clone())?;

            receive_started(id.clone())?;

            state::modify_state(id,state::Status::Running)

        }
        _ => Err(format!("Error: Status of container is not Created, cannot start")),
    }
}

/// Sends a start message to the container via a socket
fn send_start(id: String) -> Result<(),String> {
    let input_path = format!("{id}_input");
            let mut input_stream;
            match UnixStream::connect(input_path.as_str()) {
                Ok(s) => input_stream = s,
                Err(e) => return Err(format!("Error: Could not connect to inputsocket host side:\n{e}\n")),
            }
            match input_stream.write(b"Start") {
                Ok(_) =>  Ok(()),
                Err(e) => return Err(format!("Error: Failed to write on stream host side:\n{e}\n")),
            }
}

/// Listens for a "Started" message from the container on a socket
fn receive_started(id: String) -> Result<(),String> {
    let output_path = format!("{id}_output");
            let mut output_stream;
            match UnixStream::connect(output_path.as_str()) {
                Ok(s) => output_stream = s,
                Err(e) => return Err(format!("Error: Could not connect to output socket host side:\n{e}\n")),
            }
            let mut message = String::new();
            match output_stream.read_to_string(&mut message) {
                Ok(_) => (),
                Err(e) => return Err(format!("Error: Could not get response host side:\n{e}\n")),
            }
            match message.as_str() {
                "Started" => Ok(()),
                _ => Err(format!("Error: Could not start process")),
            }

}


/// Listens for a "Start" message from the host on a socket
fn receive_start(id:String) -> Result<(),String> {
    let input_path = format!("{id}_input");
    let mut input_stream;
    match UnixStream::connect(input_path.as_str()) {
        Ok(s) => input_stream = s,
        Err(e) => return Err(format!("Error: Could not connect to input socket container side:\n{e}\n")),
    }
    let mut message = String::new();
    match input_stream.read_to_string(&mut message) {
        Ok(_) => (),
        Err(e) => return Err(format!("Error: Could not get response container side:\n{e}\n")),
    }
    match message.as_str() {
        "Start" => Ok(()),
        _ => Err(format!("Error: Could not start process container side")),
    }

}

/// Sends a "Started" message to the host via a socket
fn send_started(id: String) -> Result<(),String> {
    let output_path = format!("{id}_output");
            let mut output_stream;
            match UnixStream::connect(output_path.as_str()) {
                Ok(s) => output_stream = s,
                Err(e) => return Err(format!("Error: Could not connect to inputsocket container side:\n{e}\n")),
            }
            match output_stream.write(b"Start") {
                Ok(_) =>  Ok(()),
                Err(e) => return Err(format!("Error: Failed to write on stream container side:\n{e}\n")),
            }
}