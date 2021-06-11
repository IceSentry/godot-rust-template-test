use anyhow::{bail, Result};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use log::{error, info};
use quote::format_ident;
use std::{collections::HashMap, fmt::Display, fs::File, path::Path, process::Command};
use syn::{parse_quote, visit_mut::VisitMut, Block};

pub enum GenerateType {
    Class {
        class_name: String,
        node_type: String,
    },
}

pub fn generate(generate_type: GenerateType) -> Result<()> {
    match generate_type {
        GenerateType::Class {
            class_name,
            node_type,
        } => generate_class(class_name, node_type),
    }
}

fn generate_class(class_name: String, node_type: String) -> Result<()> {
    // TODO validate project structure
    // use the `which` crate to detect if godot is available in the path

    let project_files: Vec<&str> = vec![
        "Cargo.toml",
        "godot/default_env.tres",
        "godot/export_presets.cfg",
        "godot/native/game.gdnlib",
        "godot/project.godot",
        "rust/src/lib.rs",
        "rust/Cargo.toml",
    ];

    let is_valid_project = project_files.iter().all(|i| {
        let exists = Path::new(i).exists();
        error!("{} not found", i);
        exists
    });

    if !is_valid_project {
        error!("Project format is not valid");
        bail!("Project format is not valid");
    }

    info!(
        "Generating new class {} : {}",
        class_name.to_case(Case::Pascal),
        node_type
    );

    let mut data = HashMap::new();
    data.insert("class_name", class_name.to_case(Case::Pascal));
    data.insert("node_type", node_type);

    generate_and_write_file(
        include_str!("../templates/rust_class.hbs"),
        &data,
        format!("./rust/src/{}.rs", class_name.to_case(Case::Snake)),
    )?;

    generate_and_write_file(
        include_str!("../templates/gdns_class.hbs"),
        &data,
        format!("./godot/native/{}.gdns", class_name.to_case(Case::Pascal)),
    )?;

    update_lib(class_name.to_case(Case::Snake))?;

    Ok(())
}

fn generate_and_write_file<P>(
    template_string: &str,
    data: &HashMap<&str, String>,
    file_path: P,
) -> Result<()>
where
    P: AsRef<Path> + Display,
{
    info!("Generating {}...", file_path);
    let handlebars = Handlebars::new();
    handlebars.render_template_to_write(template_string, data, File::create(file_path)?)?;
    Ok(())
}

struct Visitor {
    mod_name: String,
    class_name: String,
}

impl VisitMut for Visitor {
    fn visit_block_mut(&mut self, i: &mut Block) {
        let mod_name = format_ident!("{}", self.mod_name);
        let class_name = format_ident!("{}", self.class_name);
        i.stmts.push(parse_quote! {
            handle.add_class::<#mod_name::#class_name>();
        });
    }
}

fn write_and_fmt<P: AsRef<Path>, S: ToString>(path: P, code: S) -> Result<()> {
    std::fs::write(&path, code.to_string())?;
    Command::new("rustfmt").arg(path.as_ref()).spawn()?.wait()?;
    Ok(())
}

fn update_lib(mod_name: String) -> Result<()> {
    info!("Updating lib.rs");
    let source = std::fs::read_to_string("./rust/src/lib.rs")?;
    let mut syntax = syn::parse_file(source.as_str())?;
    let mut visitor = Visitor {
        mod_name: mod_name.to_case(Case::Snake),
        class_name: mod_name.to_case(Case::Pascal),
    };
    visitor.visit_file_mut(&mut syntax);

    write_and_fmt("./rust/src/lib.rs", quote::quote!(#syntax)).expect("unable to save or format");

    Ok(())
}
