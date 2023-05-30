use nix::{mount::{mount, umount2, MntFlags, MsFlags}, 
sched::{clone, unshare, CloneFlags}, 
unistd::{chdir, pivot_root,Pid}};
use fs_extra::copy_items;
use std::{error::Error, fs::File, io::Write};
use std::fs::create_dir_all;
use std::path::Path;
use crate::parse::{create_config,ContainerConfig};
use crate::state::{FOLDER_SUFF,STATUS_SUFF,MAIN_PATH};

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

    create_container_folder(id.clone(),&config)?;

    let pid = create_container_proc(|| {
        init_container_fs(Path::new(&config.root)).unwrap();
        use std::process;
        println!("My pid is {}", process::id());
        0
    }, config.linux.namespaces);
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
pub fn init_container_fs(new_root: &Path) -> Result<(), Box<dyn Error>> {
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )?;
    mount(
        Some(new_root),
        new_root,
        None::<&Path>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&Path>,
    )?;
    chdir(Path::new(new_root))?;
    create_dir_all(new_root.join("oldroot"))?;
    pivot_root(new_root.as_os_str(), new_root.join("oldroot").as_os_str())?;
    umount2("./oldroot", MntFlags::MNT_DETACH)?;
    chdir("/")?;
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

fn create_container_folder(id: String, config: &ContainerConfig) -> Result<(),String> {
    Ok(())
}