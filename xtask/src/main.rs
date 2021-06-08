use anyhow::{bail, Result};
use xshell::{cmd, cp, mkdir_p};

const NAME: &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> Result<()> {
    println!("{}", NAME);
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("run") => run(),
        Some("edit") => edit(),
        task => {
            bail!("Uknown Task: {:?}", task)
        }
    }
}

fn run() -> Result<()> {
    let target = "x86_64-pc-windows-msvc";
    let project_name = "godot_rust_template_test";

    // build the library
    cmd!("cargo build --target {target}").run()?;

    mkdir_p(format!("./lib/{}", target))?;
    // copy the .dll to the lib directory
    cp(
        format!("./target/{}/debug/{}.dll", target, project_name),
        format!("./lib/{}", target),
    )?;

    // start godot
    cmd!("godot --path godot/ -d").run()?;
    Ok(())
}

fn edit() -> Result<()> {
    // open the godot editor with the current project
    cmd!("godot --path godot/ -e &").run()?;
    Ok(())
}
