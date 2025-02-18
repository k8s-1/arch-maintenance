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


use crate::utils::run_command;
use std::fs;
use std::time::{Duration, SystemTime};

pub fn is_mirrorlist_up_to_date(path: &str) -> bool {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .map_or(false, |modified_time| {
            SystemTime::now()
                .duration_since(modified_time)
                .map_or(false, |duration| duration < Duration::new(604800, 0))
            // 604800 seconds = 1 week
        })
}

pub fn update_mirrorlist(path: &str) -> bool {
    let args = [
        "reflector",
        "--verbose",
        "--latest",
        "10",
        "--sort",
        "score",
        "--connection-timeout",
        "3",
        "--protocol",
        "https",
        "--save",
        path,
    ];

    run_command("sudo", &args)
}
