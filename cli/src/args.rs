use std::path::Path;

use clap::Parser;

use crate::step::{InstallAction, ResizeMethod};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the app to be generated
    ///
    /// Used as its display name in the dock and app launcher.
    #[arg(long, short = 'n', value_parser = parse_name)]
    pub name: String,

    /// Path to an animated GIF to be used as the icon of the app
    ///
    /// If the with and height of the image are not equal,
    /// the image will be resized to a square according to the `resize-method`.
    #[arg(long, short = 'g', value_parser = parse_gif)]
    pub gif: String,

    /// Shell command to be executed when the generated app is clicked
    #[arg(long, short = 'c')]
    pub command: String,

    /// Specifies how the given GIF will be resized to square
    ///
    /// `center-crop` crops the center of the image with a square
    /// whose sides are the same size as the shorter side of the image,
    /// and `center-fit` puts the image on the center of a transparent square
    /// whose sides are the same size as the longer side of the image.
    #[arg(long, short = 'm', value_enum, default_value_t = ResizeMethod::CenterFit)]
    pub resize_method: ResizeMethod,

    /// Path to a directory the app to be installed
    #[arg(long, default_value_t = default_install_location())]
    pub install_location: String,

    /// Specify the action to be taken when the app is successfully installed
    ///
    /// `launch` launches the app immediately, `open-in-finder`
    /// opens the directory where the app is installed in the Finder,
    /// and `none` does nothing. The default is `launch`.
    #[arg(long, value_enum, default_value_t = InstallAction::Launch)]
    pub install_action: InstallAction,

    /// Noisy logging, including all shell commands executed
    #[arg(long, short = 'v')]
    pub verbose: bool,

    /// Path to a working directory
    ///
    /// This is for development.
    #[arg(long)]
    pub work_dir: Option<String>,

    /// Path to a local repository
    ///
    /// This is for development.
    #[arg(long)]
    pub local_repository: Option<String>,
}

fn default_install_location() -> String {
    dirs::home_dir()
        .unwrap()
        .join("Applications")
        .to_str()
        .unwrap()
        .to_string()
}

fn parse_name(input: &str) -> Result<String, String> {
    if input.is_empty() {
        Err("app name must not be empty".to_string())
    } else {
        Ok(input.to_string())
    }
}

fn parse_gif(input: &str) -> Result<String, String> {
    let path = Path::new(input);
    match (path.try_exists(), path.is_file()) {
        (Ok(true), true) => Ok(input.to_string()),
        (Ok(true), false) => Err("Specify a GIF, not a directory".to_string()),
        (Ok(false), _) => Err("No such file".to_string()),
        (Err(error), _) => Err(error.to_string()),
    }
}
