use std::path::{Path, PathBuf};

use anyhow::{ensure, Ok, Result};
use cmd_lib::{log::info, run_cmd};
use log::debug;

use crate::utils::{self, cargo};

pub fn get_template_project<S: AsRef<Path>, D: AsRef<Path>>(
    local_repository: Option<S>,
    dest: D,
) -> Result<PathBuf> {
    info!("Getting the app template...");
    let repository = match local_repository {
        Some(path) => {
            info!("️️Use the local repository: {}", path.as_ref().display());
            PathBuf::from(path.as_ref())
        }
        None => {
            info!(
                "Downloading the remote repository...: {}",
                cargo::PKG_REPOSITORY
            );
            download_repository(&dest)?
        }
    };

    let template = repository.join("app_template");
    ensure!(template.try_exists()?);

    debug!(
        "Copying {} to {}",
        template.display(),
        dest.as_ref().display()
    );
    utils::fs::copy_dir(&template, dest)
}

fn download_repository<P: AsRef<Path>>(dest: P) -> Result<PathBuf> {
    let path = utils::fs::temp_dir_in(dest);
    let repository = cargo::PKG_REPOSITORY;
    run_cmd!(git clone $repository $path)?;
    Ok(path)
}
