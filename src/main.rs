use colored::*;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Status {
    mirror: String,
    keys: String,
    prune: String,
    orphans: String,
    cache: String,
    docker: String,
    rust: String,
}

fn main() {
    let check = "✅";
    let cross = "❌";

    let mut status = Status::default();

    println!("{}", "Updating mirror list...".yellow());
    if run_command(
        "sudo",
        &[
            "reflector",
            "--verbose",
            "--latest",
            "10",
            "--sort",
            "rate",
            "--save",
            "/etc/pacman.d/mirrorlist",
        ],
    ) {
        status.mirror = format!("{} mirror list updated", check.green());
    } else {
        status.mirror = format!("{} mirror list update failed", cross.red());
    }

    println!("{}", "Updating packages and keys...".yellow());
    if run_command("yay", &["--noconfirm"]) {
        status.keys = format!("{} packages updated", check.green());
    } else if run_command("sudo", &["pacman-keys", "--refresh-keys"])
        && run_command("yay", &["--noconfirm"])
    {
        status.keys = format!("{} packages updated and keys refreshed", check.green());
    } else {
        status.keys = format!("{} package update and key refresh failed", cross.red());
    }

    // Wrap status in Arc<Mutex> for thread-safe access
    let status = Arc::new(Mutex::new(Status::default()));

    // Helper to spawn threads for each task
    let spawn_task = |task: Box<dyn FnOnce() + Send>| {
        let handle = thread::spawn(task);
        handle
    };

    let status_clone = Arc::clone(&status);
    let prune_handle = spawn_task(Box::new(move || {
        println!("{}", "Pruning cache...".yellow());
        let result = if run_command("sudo", &["paccache", "-rk1"]) {
            format!("{} cache pruned", check.green())
        } else {
            format!("{} cache prune failed", cross.red())
        };
        status_clone.lock().unwrap().prune = result;
    }));

    let status_clone = Arc::clone(&status);
    let orphans_handle = spawn_task(Box::new(move || {
        println!("{}", "Removing orphaned packages...".yellow());
        let orphaned_packages = get_orphaned_packages();
        let result = if !orphaned_packages.is_empty()
            && run_command(
                "sudo",
                &["pacman", "-Rns", &orphaned_packages, "--noconfirm"],
            ) {
            format!("{} orphaned packages removed", check.green())
        } else if orphaned_packages.is_empty() {
            format!("{} no orphaned packages found", check.green())
        } else {
            format!(
                "{} failed to remove orphaned packages: {}",
                cross.red(),
                orphaned_packages
            )
        };
        status_clone.lock().unwrap().orphans = result;
    }));

    let status_clone = Arc::clone(&status);
    let cache_handle = spawn_task(Box::new(move || {
        println!("{}", "Cleaning cache directories...".yellow());
        let result = if run_command("rm", &["-rf", "~/.cache/*"])
            && run_command("sudo", &["rm", "-rf", "/tmp/*"])
        {
            format!("{} cache cleaned", check.green())
        } else {
            format!("{} cache directory clean-up failed", cross.red())
        };
        status_clone.lock().unwrap().cache = result;
    }));

    let status_clone = Arc::clone(&status);
    let docker_handle = spawn_task(Box::new(move || {
        println!("{}", "Cleaning Docker objects...".yellow());
        let result = if run_command("docker", &["system", "prune", "-af"]) {
            format!("{} docker cleaned", check.green())
        } else {
            format!("{} docker clean-up failed", cross.red())
        };
        status_clone.lock().unwrap().docker = result;
    }));

    let status_clone = Arc::clone(&status);
    let rust_handle = spawn_task(Box::new(move || {
        println!("{}", "Updating rust...".yellow());
        let result = if run_command("rustup", &["update"]) {
            format!("{} rust updated", check.green())
        } else {
            format!("{} rust update failed", cross.red())
        };
        status_clone.lock().unwrap().rust = result;
    }));

    // Wait for all threads to finish
    let _ = prune_handle.join();
    let _ = orphans_handle.join();
    let _ = cache_handle.join();
    let _ = docker_handle.join();
    let _ = rust_handle.join();

    // Print final status
    println!("{:<15}  {:<40}", "Item".yellow(), "Result".yellow());
    let final_status = status.lock().unwrap();
    let fields = [
        ("Mirror", &final_status.mirror),
        ("Keys", &final_status.keys),
        ("Prune", &final_status.prune),
        ("Orphans", &final_status.orphans),
        ("Cache", &final_status.cache),
        ("Docker", &final_status.docker),
        ("Rust", &final_status.rust),
    ];
    for (name, value) in fields.iter() {
        println!("{:<15}  {:<40}", name, value);
    }
}

fn run_command(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn get_orphaned_packages() -> String {
    let output = Command::new("sudo")
        .args(&["pacman", "-Qtdq"])
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
