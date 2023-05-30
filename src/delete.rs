use crate::state::build_status;
use crate::state::{MAIN_PATH, STATUS_SUFF};
use std::fs::{remove_dir_all, remove_file};

/// Deletes the container status file and the container root folder.
pub fn delete(id: String) -> Result<(), String> {
    let status = build_status(id.clone())?;
    match remove_dir_all(status.bundle.as_str()) {
        Ok(_) => (),
        Err(e) => {
            return Err(format!(
                "Error: Error while deleting container folder:\n{e}\n"
            ))
        }
    }

    match remove_file(format!("{MAIN_PATH}{STATUS_SUFF}{id}.json").as_str()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error: Error while deleting status_file:\n{e}\n")),
    }
}
