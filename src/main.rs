use anyhow::Error;
use clap::Parser;
use serde::Serialize;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
struct FlatpakManifest {
    app_id: String,
    runtime: String,
    runtime_version: String,
    sdk: String,
    command: String,
    finish_args: Vec<String>,
    modules: Vec<FlatpakModule>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
struct FlatpakModule {
    name: String,
    #[serde(rename = "buildsystem")]
    build_system: String,
    build_commands: Vec<String>,
    sources: Vec<FlatpakSource>,
}

#[derive(Serialize, Debug, Clone)]
struct FlatpakSource {
    r#type: FlatpakSourceType,
    path: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum FlatpakSourceType {
    Dir,
}

fn flatpak_id_for_game(game_id: &str) -> String {
    format!("edu.rit.csh.devcade.game.id-{game_id}")
}

async fn locate_executable(path: &Path) -> Result<String, Error> {
    // Infer executable name from *.runtimeconfig.json
    for entry in std::fs::read_dir(path)? {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(filename) = path.file_name().map(|s| s.to_str().unwrap_or("")) {
            if !filename.ends_with(".runtimeconfig.json") {
                continue;
            }
            log::debug!("Found runtimeconfig.json file: {}", filename);
            let executable = filename
                .strip_suffix(".runtimeconfig.json")
                .unwrap()
                .to_string();
            log::info!(
                "Executable inferred from runtimeconfig.json: {}",
                executable
            );
            return Ok(executable);
        }
    }

    // If no *.runtimeconfig.json file is found, look for a file with the same name as the game
    // (this is the case for games that don't use .NET)
    // TODO: Some better way to find executable name?
    // This parent().unwrap() is safe because the path is guaranteed to have a parent
    Err(anyhow::anyhow!(
        "No runtimeconfig.json found, executable names from game name are not supported sorry"
    ))
}

async fn build_flatpak(game_id: &str, game_dir: &Path) -> Result<(), Error> {
    log::info!("Preparing to build flatpak for {game_id} @ {game_dir:?}");
    let executable = locate_executable(&game_dir.join("publish")).await?;

    {
        let executable_path = game_dir.join("publish").join(&executable);
        // Chmod +x the executable
        let mut perms = executable_path.metadata()?.permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(executable_path.clone(), perms).await?;
    }

    let flatpak_manifest = FlatpakManifest {
        app_id: flatpak_id_for_game(game_id),
        runtime: "org.freedesktop.Platform".to_owned(),
        runtime_version: "22.08".to_owned(),
        sdk: "org.freedesktop.Sdk".to_owned(),
        command: format!("/app/publish/{executable}"),
        finish_args: vec![
            "--share=ipc".to_owned(),
            "--socket=x11".to_owned(),
            "--socket=pulseaudio".to_owned(),
            "--share=network".to_owned(),
            "--device=dri".to_owned(),
            "--filesystem=/tmp/devcade/persistence.sock".to_owned(),
        ],
        modules: vec![FlatpakModule {
            name: game_id.to_string(),
            build_system: "simple".to_owned(),
            build_commands: vec!["cp -r . /app/publish".to_owned()],
            sources: vec![FlatpakSource {
                r#type: FlatpakSourceType::Dir,
                path: "publish".to_owned(),
            }],
        }],
    };

    log::debug!("Writing flatpak yaml");
    let flatpak_path = game_dir.join("flatpak.yml");
    tokio::fs::write(&flatpak_path, serde_yaml::to_string(&flatpak_manifest)?).await?;
    let repo_path = game_dir.join("repo");
    let repo_path = repo_path.to_str().unwrap();

    log::info!("Building flatpak...");
    Command::new("flatpak-builder")
        .arg(format!(
            "--state-dir={}",
            game_dir.join("state-dir").to_str().unwrap()
        ))
        .arg(format!("--repo={}", repo_path))
        .arg("--force-clean")
        .arg("--user")
        .arg(game_dir.join("build").to_str().unwrap())
        .arg(flatpak_path.to_str().unwrap())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
    let bundle_path = game_dir.join("bundle.flatpak");
    let bundle_path = bundle_path.to_str().unwrap();
    Command::new("flatpak")
        .arg("build-bundle")
        .arg(repo_path)
        .arg(bundle_path)
        .arg(flatpak_manifest.app_id)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
    log::info!("Built flatpak! Output written to {bundle_path}");

    Ok(())
}

/// Converts a Devcade game directory to a Flatpak bundle
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// UUID of your game
    game_id: String,

    /// Path to the folder containing your `publish` directory
    #[arg(default_value = "./")]
    game_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    let mut builder = env_logger::Builder::new();
    builder.filter(Some("devcade_flatpakify"), log::LevelFilter::Info);
    builder.init();
    let args = Args::parse();
    build_flatpak(&args.game_id, &args.game_dir).await.unwrap();
}
