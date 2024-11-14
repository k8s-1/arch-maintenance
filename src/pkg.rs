pub fn get_orphaned_packages() -> String {
    let output = Command::new("sudo")
        .args(&["pacman", "-Qtdq"])
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
