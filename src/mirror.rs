use std::fs;
use std::time::{Duration, SystemTime};

use crate::run_command;

pub fn is_mirrorlist_up_to_date(path: &str) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => match metadata.modified() {
            Ok(modified_time) => {
                let duration_since_modified = SystemTime::now().duration_since(modified_time);
                let week_in_seconds: u64 = 604800;
                duration_since_modified
                    .map(|duration| duration < Duration::new(week_in_seconds, 0))
                    .unwrap_or(false)
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
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
        "rate",
        "--save",
        path,
    ];

    run_command("sudo", &args)
}
