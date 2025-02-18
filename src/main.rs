/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.    
*/


mod mirror;
mod utils;

use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::thread;
use std::process::{Command, Stdio};

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

const CHECK: &str = "✅";
const CROSS: &str = "❌";

fn main() {
    let status = Arc::new(Mutex::new(Status::default()));

    {
        let mut status_lock = status.lock().unwrap();

        let mirror_list_path = "/etc/pacman.d/mirrorlist";

        if !mirror::is_mirrorlist_up_to_date(mirror_list_path) {
            println!("{}", "Updating mirror list...".yellow());
            if mirror::update_mirrorlist(mirror_list_path) {
                status_lock.mirror = format!("{CHECK} mirror list updated");
            } else {
                status_lock.mirror = format!("{CROSS} mirror list update failed");
            }
        } else {
            println!("{}", "".green());
            status_lock.mirror = format!("{CHECK} mirror list is up-to-date");
        }

        println!("{}", "Updating packages and keys...".yellow());
        if utils::run_command("yay", &["--noconfirm"]) {
            status_lock.packages = format!("{CHECK} packages updated");
        } else if utils::run_command("sudo", &["pacman-keys", "--refresh-keys"])
            && utils::run_command("yay", &["--noconfirm"])
        {
            status_lock.packages = format!("{CHECK} packages updated and keys refreshed");
        } else {
            status_lock.packages = format!("{CHECK} package update and key refresh failed");
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

            let list_orphans_child = Command::new("sudo")
                .arg("pacman -Qtdq")
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start orphaned packages list process.");

            let list_orphans_out = list_orphans_child.stdout.expect("Failed to open orphaned packages list output.");

            Command::new("xargs")
                .arg("sudo pacman -Rns --noconfirm")
                .stdin(Stdio::from(list_orphans_out))
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to start orhpaned package removal process");

            format!("{CHECK} orphaned package removal complete")
        }),
    );

    let prune_handle = spawn_task(
        Arc::clone(&status),
        "prune",
        Box::new(|| {
            run_task(
                "pruning cache...",
                vec![("sudo", &["paccache", "-rk1"])],
            )
        }),
    );

    let cache_handle = spawn_task(
        Arc::clone(&status),
        "cache",
        Box::new(|| {
            run_task(
                "cleaning cache directories...",
                vec![
                    ("rm", &["-rf", "~/.cache/*"]),
                    ("sudo", &["rm", "-rf", "/tmp/*"]),
                ],
            )
        }),
    );

    let docker_handle = spawn_task(
        Arc::clone(&status),
        "docker",
        Box::new(|| {
            run_task(
                "cleaning docker objects...",
                vec![("docker", &["system", "prune", "-af"])],
            )
        }),
    );

    let rust_handle = spawn_task(
        Arc::clone(&status),
        "rust",
        Box::new(|| {
            run_task(
                "updating rust...",
                vec![("rustup", &["update"])],
            )
        }),
    );

    let _ = prune_handle.join();
    let _ = orphans_handle.join();
    let _ = cache_handle.join();
    let _ = docker_handle.join();
    let _ = rust_handle.join();

    utils::print_status(&status);
}

fn run_task(
    description: &str,
    commands: Vec<(&str, &[&str])>,
) -> String {
    println!("{}", description.yellow());

    let success = commands
        .iter()
        .all(|(cmd, args)| utils::run_command(cmd, args));

    if success {
        format!("{CHECK} {description} succeeded")
    } else {
        format!("{CROSS} {description} failed")
    }
}
