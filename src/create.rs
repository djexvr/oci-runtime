use nix::{mount::{mount, umount2, MntFlags, MsFlags},
sched::{clone, CloneFlags}, 
unistd::{chdir, pivot_root,Pid}};
use std::{error::Error, fs::File, io::Write};
use std::fs;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::Path;
use std::process::Command;
use crate::parse::create_config;
use crate::state::{FOLDER_SUFF,STATUS_SUFF,MAIN_PATH};
use crate::start::{receive_start, send_started};

pub fn to_flag(namespace: &String) -> CloneFlags {
        match namespace.as_str() {

            "pid" => CloneFlags::CLONE_NEWPID,
            "network" => CloneFlags::CLONE_NEWNET,
            "mount" => CloneFlags::CLONE_NEWNS,
            "ipc" => CloneFlags::CLONE_NEWIPC,
            "uts" => CloneFlags::CLONE_NEWUTS,
            "user" => CloneFlags::CLONE_NEWUSER,
            "cgroup" => CloneFlags::CLONE_NEWCGROUP,
            _ => CloneFlags::empty(),
        }
}

pub fn create(id: String, path: String) -> Result<(), String> {

    check_id_unicity(id.clone())?;

    let config = create_config(path)?;
    let container_fs_path_string = format!("{MAIN_PATH}{FOLDER_SUFF}{}",id.clone());
    let container_fs_path = Path::new(container_fs_path_string.as_str());
    if container_fs_path.exists() {
        remove_dir_all(container_fs_path_string.as_str()).unwrap();
    }
    std::fs::create_dir_all(&container_fs_path).unwrap();
    copy_dir::copy_dir( Path::new(&config.root), &container_fs_path).unwrap();

    
    // closure that executes the pivot_root, waits for the start message, forks for the main process, then send started message
    let pivot_root_closure = || {
        pivot_to_container_fs(&container_fs_path).unwrap();
        match receive_start(id.clone()) {
            Ok(_) => (),
            Err(s) => {println!("{s}");return -1}
        };

        match Command::new(config.process.cwd.clone())
                .args(config.process.args.clone())
                .spawn() {
            Ok(_) => (),
            Err(e) => {{println!("Could not start desired process in container:\n{e}\n");return -1}}
        }
        match send_started(id.clone()) {
            Ok(_) => (),
            Err(s) => {println!("{s}");return -1}
        };
        return 0;

        
    };

    let pid =create_container_proc(pivot_root_closure, config.linux.namespaces);
    create_status_file(id,pid)
}


// Create a new process with required namespaces using clone
pub fn create_container_proc(child_fun: impl Fn() -> isize, namespaces: Vec<String>) -> Pid {

    const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MB
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let child = clone(
        Box::new(child_fun),
        stack,
        namespaces
            .iter()
            .fold(CloneFlags::empty(), |acc, ns| acc | to_flag(ns)),
        None,
    )
    .unwrap();
    println!("Child has pid {}", child);
    child
}

// Pivot root to the given path
pub fn pivot_to_container_fs(new_root: &Path) -> Result<(), Box<dyn Error>> {
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    ).unwrap();
    mount(
        Some(new_root),
        new_root,
        None::<&Path>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&Path>,
    ).unwrap();
    chdir(Path::new(new_root)).unwrap();
    create_dir_all(new_root.join("oldroot")).unwrap();
    pivot_root(new_root.as_os_str(), new_root.join("oldroot").as_os_str()).unwrap();
    umount2("./oldroot", MntFlags::MNT_DETACH).unwrap();
    chdir("/").unwrap();
    Ok(())
}

fn create_status_file(id: String, pid: Pid) -> Result<(),String> {
    match Path::new(format!("{MAIN_PATH}{STATUS_SUFF}").as_str()).try_exists() {
        Ok(true) => (),
        _ => (),
    }
    let path_string = format!("{MAIN_PATH}{STATUS_SUFF}{id}.json");
    let path = Path::new(path_string.as_str());
    let mut file: File;
    match File::create(path) {
        Err(e) => return Err(format!("Error: Could not create status file:\n{e}")),
        Ok(f) => file = f,
    }
    let folder_path = format!("{MAIN_PATH}{FOLDER_SUFF}{id}");
    let json_content = format!("{{
        \"pid\":{},
        \"bundle\":\"{folder_path}\",
        \"status\":\"created\"
    }}",pid);
    let buf = json_content.as_bytes();
    match file.write(buf) {
        Err(e) => Err(format!("Error: Could not write to status file:\n{e}")),
        Ok(_) => Ok(()),
    }
}

fn check_id_unicity(id: String) -> Result<(),String> {
    match Path::new(format!("{MAIN_PATH}{STATUS_SUFF}{id}.json",).as_str()).try_exists() {
        Ok(true) => Err(format!("Error: Container with same ID already exists")),
        Ok(false) => Ok(()),
        Err(e) => Err(format!("Error: Unable to check for status file:\n{e}")),
    }
}