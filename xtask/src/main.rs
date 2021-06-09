use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use xshell::{cmd, cp, mkdir_p};

const PROJECT_NAME: &str = "godot_rust_template_test";
const TARGET: &str = "x86_64-pc-windows-msvc";
const TARGET_DIR: &str = "./target";

fn main() -> Result<()> {
    let task = std::env::args().nth(1);
    let flag = std::env::args().nth(2);
    let release = matches!(flag.as_deref(), Some("--release"));
    match task.as_deref() {
        Some("build") => build(release),
        Some("copy") => copy(release),
        Some("edit") => edit(),
        Some("run") => run(release),
        Some("watch") => watch(),
        task => {
            bail!("Uknown Task: {:?}", task)
        }
    }
}

/// Build the rust library and copy it to the lib directory to be used by godot
fn build(release: bool) -> Result<()> {
    // build the library
    if release {
        cmd!("cargo build --target {TARGET} --release").run()?;
    } else {
        cmd!("cargo build --target {TARGET}").run()?;
    }
    copy(release)
}

/// Copy the compiled rust library to the lib directory used by godot
fn copy(release: bool) -> Result<()> {
    let mut target_dir = PathBuf::from(TARGET_DIR);
    target_dir.push(TARGET);
    target_dir.push(if release { "release" } else { "debug" });
    target_dir.push(format!("{}.dll", PROJECT_NAME));

    // create the required folder if it doesn't already exist
    mkdir_p(format!("./lib/{}", TARGET))?;
    // copy the .dll to the lib directory
    cp(target_dir, format!("./lib/{}", TARGET))
        .with_context(|| "Failed to copy library to lib directory".to_string())
}

/// Launch the godot editor with the current project
fn edit() -> Result<()> {
    cmd!("godot --path godot/ -e &")
        .run()
        .with_context(|| "Failed to open godot editor".to_string())
}

/// Builds the rust project then launchs the game with godot
fn run(release: bool) -> Result<()> {
    build(release)?;
    cmd!("godot --path godot/ -d")
        .run()
        .with_context(|| "Failed to run game".to_string())
}

fn watch() -> Result<()> {
    cmd!("cargo watch -- cargo xtask build")
        .run()
        .with_context(|| "Failed to watch".to_string())
}
