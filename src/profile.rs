extern crate fs_extra;
use std::fs;
use std::io;
use anyhow::Result;
use anyhow::Context;

/// Checks if profile exists
///
/// #Arguments
/// * `profile_name` - &String representing name of new profile
fn profile_exists(profile_name: &String) -> Result<(), anyhow::Error> {
    let config_home = dirs::config_dir().context("could not find config home!")?
    .join("ricer_rust/profiles").join(profile_name);

    if !config_home.exists() {
        anyhow::bail!("profile does not exist!");
    }

    Ok(())
}

/// Creates a new profile directory of given name
///
/// #Arguments
/// * `profile_name` - &String representing name of new profile
fn add_profile(profile_name: &String) -> Result<(), anyhow::Error> {
    let config_home = dirs::config_dir().context("could not find config home!")?;
    fs::create_dir(config_home.join(String::from("ricer_rust/profiles/") + profile_name))?;
    Ok(())
}

/// Removes a profile
/// 
/// # Arguments
/// * `profile_name` - String representing the name of the profile to remove 
fn del_profile(profile_name: &String) -> Result<(), anyhow::Error> {
    let removal_candidate = dirs::config_dir().context("could not find config home!")?
        .join("ricer_rust").join("profiles").join(profile_name);

    fs::remove_dir_all(removal_candidate)?;

    Ok(())
}

/// Applies a profile by copying its contents into current_profile, overwriting recursive.
///
/// # Arguments
/// * `profile_name` - String representing the name of the profile to apply
fn apply_profile(profile_name: &String) -> Result<(), anyhow::Error> {
    let config_home = dirs::config_dir().context("could not find config home!")?
    .join("ricer_rust");
    let profile_dir = config_home.join("profiles").join(profile_name);

        if !profile_dir.exists() {
            anyhow::bail!("profile doesn't exist");
        }

    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    fs_extra::dir::copy(config_home.join("profiles").join(profile_name), config_home.join("current_profile"), &copy_options)?;

    Ok(())
}
