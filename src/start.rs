use super::state;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

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

fn send_start(id: String) -> Result<(),String> {
    let input_path = format!("{id}_input");
            let mut input_stream;
            match UnixStream::connect(input_path.as_str()) {
                Ok(s) => input_stream = s,
                Err(_) => return Err(format!("Error: Could not connect to inputsocket")),
            }
            match input_stream.write(b"Start") {
                Ok(_) =>  Ok(()),
                Err(_) => return Err(format!("Error: Failed to write on stream")),
            }
}

fn receive_started(id: String) -> Result<(),String> {
    let output_path = format!("{id}_output");
            let mut output_stream;
            match UnixStream::connect(output_path.as_str()) {
                Ok(s) => output_stream = s,
                Err(_) => return Err(format!("Error: Could not connect to output socket")),
            }
            let mut message = String::new();
            match output_stream.read_to_string(&mut message) {
                Ok(_) => (),
                Err(_) => return Err(format!("Error: Could not get response")),
            }
            match message.as_str() {
                "Started" => Ok(()),
                _ => Err(format!("Error: Could not start process")),
            }

}