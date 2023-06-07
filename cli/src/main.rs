mod args;
mod log;
mod step;
mod utils;

use ::log::{debug, error, info, trace, warn};
use anyhow::Result;
use args::Args;
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args = Args::parse();
    let log_file = log::init(args.verbose)?;
    debug!("Log file is available at {}", log_file.display());
    debug!("{:#?}", &args);
    match run(&args) {
        Ok(_) => {
            info!("Successfuly installed to {} 🎉", &args.install_location);
        }
        Err(error) => {
            error!("{:#}", error);
            warn!("You can see the full log here: {}", log_file.display());
            trace!("{}", error.backtrace());
        }
    }
    Ok(())
}

fn run(args: &Args) -> Result<()> {
    let work_dir = match args.work_dir.as_ref() {
        Some(path) => PathBuf::from(path),
        None => utils::fs::temp_dir(),
    };
    debug!("Creating the working directory at {}", work_dir.display());
    utils::fs::create_dir(&work_dir)?;
    let project = step::get_template_project(
        args.local_repository.as_ref().map(PathBuf::from),
        &work_dir,
    )?;
    step::fill_template_project(
        &project,
        &args.command,
        &args.gif,
        args.resize_method,
    )?;
    let app = step::build_app(&project, &args.name, &work_dir)?;
    step::install_app(
        app,
        PathBuf::from(&args.install_location),
        args.install_action,
    )
}
