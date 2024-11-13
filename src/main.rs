use colored::*;
use std::process::Command;

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

    println!("{}", "Pruning cache...".yellow());
    if run_command("sudo", &["paccache", "-rk1"]) {
        status.prune = format!("{} cache pruned", check.green());
    } else {
        status.prune = format!("{} cache prune failed", cross.red());
    }

    println!("{}", "Removing orphaned packages...".yellow());
    let orphaned_packages = get_orphaned_packages();
    if !orphaned_packages.is_empty()
        && run_command(
            "sudo",
            &["pacman", "-Rns", &orphaned_packages, "--noconfirm"],
        )
    {
        status.orphans = format!("{} orphaned packages removed", check.green());
    } else if orphaned_packages.is_empty() {
        status.orphans = format!("{} no orphaned packages found", check.green());
    } else {
        status.orphans = format!("{} failed to remove orphaned packages: {}", cross.red(), &orphaned_packages);
    }

    println!("{}", "Cleaning cache directories...".yellow());
    if run_command("rm", &["-rf", "~/.cache/*"]) && run_command("sudo", &["rm", "-rf", "/tmp/*"]) {
        status.cache = format!("{} cache cleaned", check.green());
    } else {
        status.cache = format!("{} cache directory clean-up failed", cross.red());
    }

    println!("{}", "Cleaning Docker objects...".yellow());
    if run_command("docker", &["system", "prune", "-af"]) {
        status.docker = format!("{} docker cleaned", check.green());
    } else {
        status.docker = format!("{} docker clean-up failed", cross.red());
    }

    println!("{}", "Updating rust...".yellow());
    if run_command("rustup", &["update"]) {
        status.rust = format!("{} rust updated", check.green());
    } else {
        status.rust = format!("{} rust update failed", cross.red());
    }

    println!("{:<15}  {:<40}", "Item".yellow(), "Result".yellow());
    let fields = [
        ("Mirror", &status.mirror),
        ("Keys", &status.keys),
        ("Prune", &status.prune),
        ("Orphans", &status.orphans),
        ("Cache", &status.cache),
        ("Docker", &status.docker),
        ("Rust", &status.rust),
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
