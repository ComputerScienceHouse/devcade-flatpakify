# Devcade Flatpakify

Converts a Devcade-formatted publish folder into a Flatpak bundle.

## Installation

### Debian

Follow these steps:
```bash
# Install dependencies
apt install flatpak-builder flatpak
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

1. Install [WSL 2](https://learn.microsoft.com/en-us/windows/wsl/install)
2. See [the steps above for Debian](#Debian)
