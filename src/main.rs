use procfs::process::Process;
use procfs::{ProcError, ProcResult};
use serde::Serialize;

#[derive(Serialize)]
struct ProcInfo {
    pid: i32,
    comm: String,
    exe: Option<String>,
    cwd: Option<String>,
    status: Option<String>,
    memory: Option<u64>,
}

fn get_proc_info(pid: i32) -> ProcResult<ProcInfo> {
    let process = Process::new(pid)?;
    // copy the comm string to a new String
    let comm: String = String::new() + &process.stat.comm;
    Ok(ProcInfo {
        pid,
        comm: comm,
        exe: process.exe().ok().map(|p| p.display().to_string()),
        cwd: process.cwd().ok().map(|p| p.display().to_string()),
        status: process.status().ok().map(|s| s.state),
        memory: Some(process.stat.vsize),
    })
}

fn main() -> Result<(), ProcError> {
    let mut processes: Vec<ProcInfo> = Vec::new();

    for process in procfs::process::all_processes()? {
        if let Ok(info) = get_proc_info(process.pid) {
            processes.push(info);
        }
    }

    // Convert to JSON
    let json = serde_json::to_string_pretty(&processes).expect("Failed to serialize processes");

    // Print JSON
    println!("{}", json);

    Ok(())
}
