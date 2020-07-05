use yaml_rust::{yaml, Yaml, YamlLoader};
use std::path::Path;
use std::fs;
use anyhow::Result;
use anyhow::Context;
use std::collections::BTreeMap;
use std::io::prelude::*;

pub mod profile;
pub mod config;

// sets up config directory if needed
// returns error if config home couldn't be retrieved, or any directory creation failed.
fn setup() -> Result<(), anyhow::Error> {
    let config_home = dirs::config_dir().context("could not find config home!")?;
    let conf_path = Path::new(&config_home).join(Path::new("ricer_rust"));

    if !conf_path.exists() {
        fs::create_dir(&conf_path).context("failed to create config dir")?;
        fs::create_dir(conf_path.join(Path::new("colors")))?;
        fs::create_dir(conf_path.join(Path::new("profiles")))?;
        fs::create_dir(conf_path.join(Path::new("modules")))?;
        fs::create_dir(conf_path.join(Path::new("current_profile")))?;
    }
    Ok(())
}

fn template(file: &Path, colors: &yaml::Hash, modules: &BTreeMap<String, String>) -> Result<String, anyhow::Error> {
    let template_txt = fs::read_to_string(file).context(format!("could not read template at {:?}", file))?;
    let mut substituted = String::new();
    for token in template_txt.split(|c| c == '{' || c == '}') {
        if token.starts_with("#") && token.len() > 1 {
            let token_key = &token[1..token.len()];
            if let Some(val) = modules.get(token_key) {
                substituted += val;
            }
            else if let Some(Yaml::String(val)) = colors.get(&Yaml::from_str(&format!("base0{}", token_key))) {
                substituted += val;
            }
            else {
                substituted += token;
            }
        }
        else {
            substituted += token;
        }
    }
    println!("{}", substituted);

    Ok(substituted)
}

fn load() -> Result<(), anyhow::Error> {
    let modules_err = "failure reading modules in conf dir";
    let current_profile_err = "failure reading current profile";

    let config_home = dirs::config_dir().context("could not find config home!")?
    .join("ricer_rust");

    let colors_dir = config_home.join("colors").join("pumpkin.yaml");
    let colors: yaml::Hash;
    if let Yaml::Hash(colors_map) = YamlLoader::load_from_str(&fs::read_to_string(colors_dir).expect("file read error").to_ascii_lowercase())?.remove(0) {
        colors = colors_map;
        if (0..16).any(|x| colors.contains_key(&Yaml::from_str(&format!("base{:0x}", x)))) {
            anyhow::bail!("required hex colors base00-base0f are not present.")
        }
    }
    else {
        anyhow::bail!("error reading the yaml");
    }

    let mut modules = BTreeMap::new();
    for dirent in fs::read_dir(config_home.join("modules")).context(modules_err)? {
        let dirent = dirent.context(modules_err)?;
        if !dirent.file_type().context(modules_err)?.is_file() {
            continue;
        }
        let mut module_txt = fs::read_to_string(dirent.path()).expect("file read error").to_ascii_lowercase();
        if let Some(last_char) = module_txt.chars().last() {
            if last_char == '\n' {
                module_txt.pop();
            }
            if let Ok(module_name) = dirent.file_name().into_string() {
                modules.insert(module_name, module_txt);
            }
        }
    }

    for dirent in fs::read_dir(config_home.join("current_profile")).context(current_profile_err)? {
        let dirent = dirent.context(current_profile_err)?;
        if dirent.file_type().context(current_profile_err)?.is_dir() && !dirent.path().join("template").is_file() && !dirent.path().join("target").is_file() {
            if let Ok(dir_name) = dirent.file_name().into_string() {
                println!("either conf template or target symlink not present or \'{}\' is not a dir", dir_name);
            }
        }
        else {
            let templated = template(&dirent.path().join("template"), &colors, &modules)?;
            let mut templated_symlink = fs::File::create(dirent.path().join("target")).context(format!("error writing to target symlink at {:?}", dirent.path()))?;
            templated_symlink.write_all(templated.as_bytes()).context(format!("error writing to target symlink at {:?}", dirent.path()))?;
        }
    }
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    setup()?;
    load()?;
    Ok(())
}
