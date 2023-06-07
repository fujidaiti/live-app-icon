use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result};
use cmd_lib::run_fun;
use log::{info, trace, debug};

use crate::utils;

pub fn build_app(
    src: &Path,
    app_name: &str,
    dest: &Path,
) -> Result<PathBuf> {
    info!("Building the app from the source...");
    let project = utils::fs::find_item_by_extension(src, extension::XCODEPROJ)?;
    let archive = archive_project(&project, app_name, &bundle_id(app_name), dest)?;
    export_archive(&archive, dest)
}

fn archive_project(
    project: &Path,
    app_name: &str,
    bundle_id: &str,
    dest: &Path,
) -> Result<PathBuf> {
    debug!("Archiving the project...");
    let scheme = find_scheme(project);
    let archive = dest.join(format!("archive.{}", extension::XCARCHIVE));
    let config = create_xcconfig(app_name, bundle_id, dest)?;
    debug!("With this build .xcconfig:");
    debug!("{}", run_fun!(cat $config)?);
    let output = run_fun!(xcodebuild archive -project $project
        -scheme $scheme -xcconfig $config -archivePath $archive)?;
    trace!("{}", output);
    Ok(archive)
}

fn export_archive(archive: &Path, dest: &Path) -> Result<PathBuf> {
    debug!("Exporting the archive: {}", archive.display());
    let options = archive.join(format!("Info.{}", extension::PLIST));
    let output = run_fun!(xcodebuild -exportArchive -archivePath $archive
        -exportPath $dest -exportOptionsPlist $options)?;
    trace!("{}", output);
    utils::fs::find_item_by_extension(dest, extension::APP)
}

fn build_config(app_name: &str, bundle_id: &str, team_id: &str) -> String {
    format!(
        "
    PRODUCT_BUNDLE_IDENTIFIER = {bundle_id}
    PRODUCT_NAME = {app_name}

    CODE_SIGN_IDENTITY =
    CODE_SIGN_IDENTITY[sdk=macosx*] = Apple Development

    CODE_SIGN_STYLE = Manual

    DEVELOPMENT_TEAM =
    DEVELOPMENT_TEAM[sdk=macosx*] = {team_id}
    "
    )
}

fn create_xcconfig(
    app_name: &str,
    bundle_id: &str,
    dest: &Path,
) -> Result<PathBuf> {
    debug!("Creating the build config...");
    let path = dest.join(format!("config.{}", extension::XCCONFIG));
    let team_id = find_apple_personal_team_id()?;
    let mut file = File::create(&path)?;
    let config = build_config(app_name, bundle_id, &team_id);
    file.write_all(config.as_bytes())?;
    Ok(path)
}

fn find_apple_personal_team_id() -> Result<String> {
    let id = run_fun!(
        security find-certificate -p -c "Apple Development" login.keychain
        | openssl x509 -noout -subject 
        | grep -o "/OU=[0-9a-zA-Z]\\+/" 
        | sed -E "s/(\\/)|(OU=)//g")?;
    Ok(id)
}

fn find_scheme(_: &Path) -> String {
    "LiveAppIcon".to_string()
}

fn bundle_id(label: &str) -> String {
    format!("dev.norelease.liveappicon.{}", label)
}

mod extension {
    pub const XCODEPROJ: &str = "xcodeproj";
    pub const APP: &str = "app";
    pub const XCARCHIVE: &str = "xcarchive";
    pub const PLIST: &str = "plist";
    pub const XCCONFIG: &str = "xcconfig";
}
