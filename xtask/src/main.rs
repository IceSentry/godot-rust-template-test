mod generator;

use anyhow::{Context, Result};
use clap::{AppSettings, Clap};
use simplelog::{ColorChoice, Config, SimpleLogger, TermLogger, TerminalMode};
use xshell::{cmd, cp, mkdir_p};

use std::path::PathBuf;

const PROJECT_NAME: &str = "godot_rust_template_test";
const TARGET: &str = "x86_64-pc-windows-msvc";
const TARGET_DIR: &str = "./target";

#[derive(Clap)]
#[clap(version = "1.0", author = "Icesentry")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    /// Build the rust library and copy it to the lib directory to be used by godot
    Build(Build),
    /// Copy the compiled rust library to the lib directory used by godot
    Copy(Copy),
    /// Launch the godot editor with the current project
    Edit,
    /// Builds the rust project then launchs the game with godot
    Run(Run),
    /// build and copy whenever a change is detected
    Watch,
    /// Generates a new class. new .rs file, new .gdns file, updates lib.rs
    NewClass(NewClass),
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Build {
    #[clap(short, long)]
    release: bool,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Copy {
    #[clap(short, long)]
    release: bool,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Run {
    #[clap(short, long)]
    release: bool,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct NewClass {
    class_name: String,
    #[clap(default_value = "Node")]
    node_type: String,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    TermLogger::init(
        log::LevelFilter::Info,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )?;
    match opts.command {
        Command::Build(args) => build(args.release),
        Command::Copy(args) => copy(args.release),
        Command::Edit => edit(),
        Command::Run(args) => run(args.release),
        Command::Watch => watch(),
        Command::NewClass(args) => new_class(args.class_name, args.node_type),
    }
}

fn build(release: bool) -> Result<()> {
    if release {
        cmd!("cargo build --target {TARGET} --release").run()?;
    } else {
        cmd!("cargo build --target {TARGET}").run()?;
    }
    copy(release)
}

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

fn edit() -> Result<()> {
    cmd!("godot --path godot/ -e &")
        .run()
        .with_context(|| "Failed to open godot editor".to_string())
}

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

fn new_class(class_name: String, node_type: String) -> Result<()> {
    generator::generate(generator::GenerateType::Class {
        class_name,
        node_type,
    })
}
