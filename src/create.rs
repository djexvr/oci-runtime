use nix::{mount::{mount, umount2, MntFlags, MsFlags}, sched::{clone, unshare, CloneFlags}, unistd::{chdir, pivot_root}};
use std::error::Error;
use std::fs::create_dir_all;
use std::path::Path;

enum Namespace {
    PID,
    Network,
    Mount,
    IPC,
    UTS,
    User,
    Cgroup,
    Time,
}

impl Namespace {
    pub fn to_flag(&self) -> CloneFlags {
        match &self {
            &PID => CloneFlags::CLONE_NEWPID,
            &Network => CloneFlags::CLONE_NEWNET,
            &Mount => CloneFlags::CLONE_NEWNS,
            &IPC => CloneFlags::CLONE_NEWIPC,
            &UTS => CloneFlags::CLONE_NEWUTS,
            &User => CloneFlags::CLONE_NEWUSER,
            &Cgroup => CloneFlags::CLONE_NEWCGROUP,
            &Time => CloneFlags::empty(), // Clone can't create a new time namespace
        }
    }
}

// Create a new process with required namespaces using clone
pub fn create_container_proc(child_fun: impl Fn() -> isize) {
    // TODO: get namespace from config
    let namespaces = vec![
        Namespace::PID,
        Namespace::Network,
        Namespace::Mount,
        Namespace::IPC,
        Namespace::UTS,
        Namespace::User,
        Namespace::Cgroup,
    ];

    const STACK_SIZE: usize = 4 * 1024 * 1024; // 4 MB
    let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let child = clone(
        Box::new(child_fun),
        stack,
        namespaces
            .iter()
            .fold(CloneFlags::empty(), |acc, ns| acc | ns.to_flag()),
        None,
    )
    .unwrap();
    println!("Child has pid {}", child);
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
    unshare(CloneFlags::CLONE_NEWNS)?; // For some reason we need to unshare even though a new namespace was created when cloning
    chdir(Path::new(new_root))?;
    create_dir_all(new_root.join("oldroot"))?;
    pivot_root(new_root.as_os_str(), new_root.join("oldroot").as_os_str())?;
    umount2("./oldroot", MntFlags::MNT_DETACH)?;
    chdir("/")?;
    Ok(())
}
