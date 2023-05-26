use super::state;
use std::fs::remove_dir_all;
use super::state::{MAIN_PATH,FOLDER_SUFF};

pub fn delete(id: String) -> Result<(),String>{
    let status =state::build_status(id.clone())?;
    match status.status {
        state::Status::Stopped => {
            match remove_dir_all(&format!("{MAIN_PATH}{FOLDER_SUFF}{id}")[..]) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!("Error: Error while deleting container folder")), 
            }
        }
        _ => return Err(format!("Error: Status of container is not Stopped, cannot delete"))
    }
}

