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


use std::process::Command;
use colored::*;
use std::sync::Mutex;

use crate::Status;

pub fn run_command(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn print_status(status: &Mutex<Status>) {
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

    let mut output = format!("{:<15}  {:<40}\n", "Item".yellow(), "Result".yellow());

    for (name, value) in fields.iter() {
        output.push_str(&format!("{:<15}  {:<40}\n", name, value));
    }
    
    println!("{}", output);
}
