# Devcade Flatpakify

Converts a Devcade-formatted publish folder into a Flatpak bundle.

## Installation

### Debian

Follow these steps:
```bash
# Update package lists
sudo apt update
# Install dependencies
sudo apt install flatpak-builder flatpak curl build-essential
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Install devcade-flatpakify
cargo install --git https://github.com/ComputerScienceHouse/devcade-flatpakify
# Add flatpak repos
flatpak remote-add --user --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
# Install SDK
flatpak install --user org.freedesktop.Sdk/x86_64/22.08 org.freedesktop.Platform/x86_64/22.08
```

If you're using some other distribubution, I trust you can figure it out :)

### Windows

1. Install Debian [WSL 2](https://learn.microsoft.com/en-us/windows/wsl/install): `wsl --install -d Debian`
2. See [the steps above for Debian](#Debian)
