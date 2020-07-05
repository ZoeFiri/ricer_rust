use std::path::Path; 
use std::fs;
use std::io;
use anyhow::Result;
use anyhow::Context;

/// Adds a config file into the templates schema
///
/// # Arguments
/// * `conf_name` - String representing the name of the program the conf belongs to
/// * `conf_path` - The original path of the config file
/// * `dest` - where to place the template, this is either the template dir or inside of a theme profile
fn add_conf(conf_name: &String, conf_path: &Path, dest: &Path) -> Result<(), anyhow::Error> {
    if !conf_path.exists() {
        anyhow::bail!("file does not exist");
    }
    
    let new_dir = dest.join(conf_name);
    fs::create_dir(&new_dir)?;
    fs::copy(&conf_path, new_dir.join("template"))?;
    std::os::unix::fs::symlink(&conf_path, new_dir.join("target"))?;
    Ok(())
}

/// Removes a program's configs from ALL profiles and current_profile
///
/// # Arguments
/// * `conf_name` - String representing the name of the program the conf belongs to
fn del_conf_global(conf_name: &String) -> Result<(), anyhow::Error> {
    let config_home = dirs::config_dir().context("could not find config home!")?
    .join("ricer_rust");

    let mut found = false;
    for entry in fs::read_dir(config_home.join("current_profile"))? {
        let dir = entry?;
        let dir_name = dir.file_name().into_string();
        if dir_name == Ok(String::from(conf_name)) {
            fs::remove_dir_all(dir.path())?;
            found = true;
        }
    }

    let current_profile_path = config_home.join(String::from("current_profile/") + conf_name);
    if let Ok(_) = fs::remove_dir_all(current_profile_path) {
        found = true;
    }
    else if !found {
        anyhow::bail!("program's config was not present anywhere");
    }

    Ok(())
}

/// Removes a program's configs from a particular profile 
///  
/// # Arguments
/// * `conf_name` - String representing the name of the program the conf belongs to
/// * `profile_name` - String representing the name of the profile to remove conf_name from
fn del_conf_profile(conf_name: &String, profile_name: &String) -> Result<(), anyhow::Error> {
    let removal_candidate = dirs::config_dir().context("could not find config home!")?
        .join("ricer_rust").join("profiles").join(profile_name).join(conf_name);

    fs::remove_dir_all(removal_candidate)?;

    Ok(())
}
