mod mirror;
mod pkg;
mod utils;

use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Status {
    mirror: String,
    packages: String,
    prune: String,
    orphans: String,
    cache: String,
    docker: String,
    rust: String,
}

fn main() {
    let (check, cross) = ("✅", "❌");
    let status = Arc::new(Mutex::new(Status::default()));

    {
        let mut status_lock = status.lock().unwrap();

        let mirror_list_path = "/etc/pacman.d/mirrorlist";

        if !mirror::is_mirrorlist_up_to_date(mirror_list_path) {
            println!("{}", "Updating mirror list...".yellow());
            if mirror::update_mirrorlist(mirror_list_path) {
                status_lock.mirror = format!("{} mirror list updated", check.green());
            } else {
                status_lock.mirror = format!("{} mirror list update failed", cross.red());
            }
        } else {
            println!("{}", "".green());
            status_lock.mirror = format!("{} mirror list is up-to-date", check.green());
        }

        println!("{}", "Updating packages and keys...".yellow());
        if utils::run_command("yay", &["--noconfirm"]) {
            status_lock.packages = format!("{} packages updated", check.green());
        } else if utils::run_command("sudo", &["pacman-keys", "--refresh-keys"])
            && utils::run_command("yay", &["--noconfirm"])
        {
            status_lock.packages = format!("{} packages updated and keys refreshed", check.green());
        } else {
            status_lock.packages = format!("{} package update and key refresh failed", cross.red());
        }
    }

    let spawn_task =
        |status: Arc<Mutex<Status>>, field: &str, task: Box<dyn FnOnce() -> String + Send>| {
            let field = field.to_string();
            thread::spawn(move || {
                let result = task();
                let mut status = status.lock().unwrap();
                match field.as_str() {
                    "prune" => status.prune = result,
                    "orphans" => status.orphans = result,
                    "cache" => status.cache = result,
                    "docker" => status.docker = result,
                    "rust" => status.rust = result,
                    _ => (),
                }
            })
        };

    let orphans_handle = spawn_task(
        Arc::clone(&status),
        "orphans",
        Box::new(|| {
            println!("{}", "Removing orphaned packages...".yellow());

            let orphaned_packages = pkg::get_orphaned_packages();

            match (
                orphaned_packages.is_empty(),
                utils::run_command(
                    "sudo",
                    &["pacman", "-Rns", &orphaned_packages, "--noconfirm"],
                ),
            ) {
                (true, _) => format!("{} no orphaned packages found", check.green()),
                (false, true) => format!("{} orphaned packages removed", check.green()),
                (false, false) => format!(
                    "{} failed to remove orphaned packages: {}",
                    cross.red(),
                    orphaned_packages
                ),
            }

        }),
    );


    let prune_handle = spawn_task(
        Arc::clone(&status),
        "prune",
        Box::new(|| {
            println!("{}", "Pruning cache...".yellow());
            if utils::run_command("sudo", &["paccache", "-rk1"]) {
                format!("{} cache pruned", check.green())
            } else {
                format!("{} cache prune failed", cross.red())
            }
        }),
    );

    let cache_handle = spawn_task(
        Arc::clone(&status),
        "cache",
        Box::new(|| {
            println!("{}", "Cleaning cache directories...".yellow());
            if utils::run_command("rm", &["-rf", "~/.cache/*"])
                && utils::run_command("sudo", &["rm", "-rf", "/tmp/*"])
            {
                format!("{} cache cleaned", check.green())
            } else {
                format!("{} cache directory clean-up failed", cross.red())
            }
        }),
    );

    let docker_handle = spawn_task(
        Arc::clone(&status),
        "docker",
        Box::new(|| {
            println!("{}", "Cleaning Docker objects...".yellow());
            if utils::run_command("docker", &["system", "prune", "-af"]) {
                format!("{} docker cleaned", check.green())
            } else {
                format!("{} docker clean-up failed", cross.red())
            }
        }),
    );

    let rust_handle = spawn_task(
        Arc::clone(&status),
        "rust",
        Box::new(|| {
            println!("{}", "Updating rust...".yellow());
            if utils::run_command("rustup", &["update"]) {
                format!("{} rust updated", check.green())
            } else {
                format!("{} rust update failed", cross.red())
            }
        }),
    );

    let _ = prune_handle.join();
    let _ = orphans_handle.join();
    let _ = cache_handle.join();
    let _ = docker_handle.join();
    let _ = rust_handle.join();

    utils::print_status(&status);
}
