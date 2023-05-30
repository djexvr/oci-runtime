use fs_extra::{copy_items, dir::CopyOptions};
use nix::{mount::{mount, umount2, MntFlags, MsFlags}, sched::{clone, unshare, CloneFlags}, unistd::{chdir, pivot_root}};
use std::error::Error;
use std::fs::create_dir_all;
use std::path::Path;
use crate::parse::create_config;

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
    let config = create_config(path)?;
    let container_fs_path = Path::new("./.oci-runtime").join(id.as_str());
    std::fs::create_dir_all(&container_fs_path).unwrap();
    copy_dir::copy_dir(&container_fs_path.join("fs"), Path::new(&config.root)).unwrap();
    create_container_proc(|| {
        pivot_to_container_fs(&container_fs_path).unwrap();
        use std::process;
        println!("My pid is {}", process::id());
        0
    }, config.linux.namespaces);
    Ok(())
}
// Create a new process with required namespaces using clone
pub fn create_container_proc(child_fun: impl Fn() -> isize, namespaces: Vec<String>) {

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
}

// Pivot root to the given path
pub fn pivot_to_container_fs(new_root: &Path) -> Result<(), Box<dyn Error>> {
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
