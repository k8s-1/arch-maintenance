use std::process::Command;
use colored::*;

#[derive(Default)]
pub struct Status {
    pub mirror: String,
    pub packages: String,
    pub prune: String,
    pub orphans: String,
    pub cache: String,
    pub docker: String,
    pub rust: String,
}

pub fn run_command(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn print_status(status: &Mutex<Status>) {
    println!("{:<15}  {:<40}", "Item".yellow(), "Result".yellow());
    let final_status = status.lock().unwrap();
    let fields = [
        ("Mirror", &final_status.mirror),
        ("Packages", &final_status.packages),
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
