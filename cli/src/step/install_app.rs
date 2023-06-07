use std::path::Path;

use anyhow::{Context, Ok, Result};
use clap::ValueEnum;
use cmd_lib::run_cmd;
use log::info;

use crate::utils;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InstallAction {
    Launch,
    OpenInFinder,
    None,
}

pub fn install_app<A: AsRef<Path>, L: AsRef<Path>>(
    app: A,
    location: L,
    action: InstallAction,
) -> Result<()> {
    info!("Installing the app...");
    let location = location.as_ref();
    let installed_app = utils::fs::copy_dir(app, location)?;

    match action {
        InstallAction::None => Ok(()),
        InstallAction::Launch => {
            run_cmd!(open $installed_app).with_context(|| {
                format!("Failed to open {}", installed_app.display())
            })
        }
        InstallAction::OpenInFinder => run_cmd!(open $location)
            .with_context(|| format!("Failed to open {}", location.display())),
    }
}
