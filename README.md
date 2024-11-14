# Arch System Maintenance Tool

This Rust-based project is a comprehensive **system maintenance tool for Arch Linux** designed to automate key maintenance tasks, keeping your system clean, optimized, and up-to-date. With a focus on concurrency, efficiency, and simplicity, this tool streamlines several tasks such as updating mirrors, cleaning caches, managing orphaned packages, and updating the system environment, including Rust and Docker.

---

## Features

- **Automatic Mirror Updates**: Refreshes and updates Arch mirrors to ensure you are always connected to the fastest, most up-to-date sources.

- **Package & Key Updates**: Executes system package updates and refreshes keys as needed, ensuring that your system remains secure and current.

- **Cache Cleanup**: Prunes and cleans package and system caches to free up space and improve performance.

- **Orphaned Package Removal**: Identifies and removes orphaned packages no longer needed by the system.

- **Docker Pruning**: Cleans up unused Docker images, containers, and volumes, ensuring Docker doesn’t consume unnecessary disk space.

- **Rust Update**:
  Keeps the Rust toolchain updated to the latest version to ensure compatibility with the latest libraries and tools.

---

## Installation ⚙️

1. **Prerequisites**:
 - Ensure that you have Rust and Cargo installed on your Arch Linux system. If not, you can install them as follows:
   ```bash
   sudo pacman -S rust
   ```
 - User must be in sudoers group

2. Clone this repository and place the build in /usr/local/bin
```bash
git clone https://github.com/yourusername/arch-maintenance-tool.git
cd arch-maintenance-tool
cargo build --release
mv ./target/release/arch-maintenance-tool /usr/local/bin/

arch-maintenance-tool
```

---

## Example output
```vbnet
> ./target/release/rust-maintenance

...
Item             Result
Mirror           ✅ mirror list is up-to-date
Packages         ✅ packages updated
Prune            ✅ pruning cache... succeeded
Orphans          ✅ no orphaned packages found
Cache            ✅ cleaning cache directories... succeeded
Docker           ✅ cleaning docker objects... succeeded
```

---

## Contributing 🤝

We welcome contributions! If you'd like to improve this tool, feel free to fork the repository and create a pull request. You can also open issues for any bugs or feature requests.

---

## License 📜

This project is licensed under the Apache License - see the [LICENSE](LICENSE) file for details.



