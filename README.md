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

## Usage

```bash
devcade-flatpakify <GAME_ID> [GAME_DIR]
```
For more information, try:
```bash
devcade-flatpakify --help
```

### To flatpak a dotnet project for devcade:

First find the GAME_ID by navigating to the website.
If you have no created a game first do so by going to **Create Game** and following the instructions there.
After creating a game go to the **Games** tab click on your game, click on **upload** and on the right under **How To Upload** the GAME_ID will be shown and will look like: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxx

Build your project if you have not done so already (make sure it is self contained):
```bash
dotnet publish -c Release -r linux-x64 --self-contained
```
Then navigate to the folder **containing** the publish directory.
The path to the directory containing the publish folder will look similar to: DevcadeGame/bin/Release/net10.0/linux-x64/ <br>
and run:
```bash
devcade-flatpackify <GAME_ID>
```
or if you want to specify the path of the directory containing the publish folder more explicitly or without cd-ing to it everytime:
```bash
devcade-flatpackify <GAME_ID> <Path-to-publish-folder>
```

