use crate::state;
use std::fs::{remove_dir_all,remove_file};
use crate::state::{MAIN_PATH,FOLDER_SUFF,STATUS_SUFF};

/// Deletes the container status file and the container root folder.
pub fn delete(id: String) -> Result<(),String>{
    let status =state::build_status(id.clone())?;
    match status.status {
        state::Status::Stopped => {
            
            match remove_dir_all(format!("{MAIN_PATH}{FOLDER_SUFF}{id}").as_str()) {
                Ok(_) => (),
                Err(_) => return Err(format!("Error: Error while deleting container folder")), 
            }

            match remove_file(format!("{MAIN_PATH}{STATUS_SUFF}{id}.json").as_str()) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!("Error: Error while deleting status_file"))
            }
        }
        _ => return Err(format!("Error: Status of container is not Stopped, cannot delete"))
    }
}

