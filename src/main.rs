use std::process::Command;
use colored::*;
use std::process::ExitStatus;

fn main() {
    let check = "✅";
    let cross = "❌";

    // Status messages
    let mut mirror_status = String::new();
    let mut keys_status = String::new();
    let mut prune_status = String::new();
    let mut orphans_status = String::new();
    let mut cache_status = String::new();
    let mut docker_status = String::new();
    let mut rkhunter_status = String::new();
    let mut rust_status = String::new();

    println!("{}", "Updating mirror list...".yellow());
    if run_command("sudo", &["reflector", "--verbose", "--latest", "10", "--sort", "rate", "--save", "/etc/pacman.d/mirrorlist"]) {
        mirror_status = format!("{} mirror list updated", check.green());
    } else {
        mirror_status = format!("{} mirror list update failed", cross.red());
    }

    println!("{}", "Updating packages and keys...".yellow());
    if run_command("yay", &["--noconfirm"]) {
        keys_status = format!("{} packages updated", check.green());
    } else if run_command("sudo", &["pacman-keys", "--refresh-keys"]) && run_command("yay", &["--noconfirm"]) {
        keys_status = format!("{} packages updated and keys refreshed", check.green());
    } else {
        keys_status = format!("{} package update and key refresh failed", cross.red());
    }

    println!("{}", "Pruning cache...".yellow());
    if run_command("sudo", &["paccache", "-rk1"]) {
        prune_status = format!("{} cache pruned", check.green());
    } else {
        prune_status = format!("{} cache prune failed", cross.red());
    }

    println!("{}", "Removing orphaned packages...".yellow());
    let orphaned_packages = get_orphaned_packages();
    if !orphaned_packages.is_empty() && run_command("sudo", &["pacman", "-Rns", &orphaned_packages, "--noconfirm"]) {
        orphans_status = format!("{} orphaned packages removed", check.green());
    } else if orphaned_packages.is_empty() {
        orphans_status = format!("{} no orphaned packages found", check.green());
    } else {
        orphans_status = format!("{} failed to remove orphaned packages", cross.red());
    }

    println!("{}", "Cleaning cache directories...".yellow());
    if run_command("rm", &["-rf", "~/.cache/*"]) && run_command("sudo", &["rm", "-rf", "/tmp/*"]) {
        cache_status = format!("{} cache cleaned", check.green());
    } else {
        cache_status = format!("{} cache directory clean-up failed", cross.red());
    }

    println!("{}", "Cleaning Docker objects...".yellow());
    if run_command("docker", &["system", "prune", "-af"]) {
        docker_status = format!("{} docker cleaned", check.green());
    } else {
        docker_status = format!("{} docker clean-up failed", cross.red());
    }

    println!("{}", "Running rkhunter checks...".yellow());
    if run_command("sudo", &["rkhunter", "--propupd"]) && run_command("sudo", &["rkhunter", "--update"]) &&
       run_command("sudo", &["rkhunter", "--check", "--sk", "--rwo", "--quiet"]) {
        rkhunter_status = format!("{} rkhunter passed", check.green());
    } else {
        rkhunter_status = format!("{} rkhunter check failed... possible security issue detected!", cross.red());
    }

    println!("{}", "Updating rust...".yellow());
    if run_command("rustup", &["update"]) {
        rust_status = format!("{} rust updated", check.green());
    } else {
        rust_status = format!("{} rust update failed", cross.red());
    }

    // Summary
    println!("\n{}", "Summary:".yellow());
    println!("{}", mirror_status);
    println!("{}", keys_status);
    println!("{}", prune_status);
    println!("{}", orphans_status);
    println!("{}", cache_status);
    println!("{}", docker_status);
    println!("{}", rkhunter_status);
    println!("{}", rust_status);
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
