use std::{fmt::Arguments, io, path::PathBuf};

use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use fern::FormatCallback;
use log::Record;

use crate::utils::{self, cargo};

pub fn init(verbose: bool) -> Result<PathBuf> {
    let display_log = if verbose {
        fern::Dispatch::new()
            .level(log::LevelFilter::Trace)
            .format(format_debug_log)
            .chain(io::stdout())
    } else {
        fern::Dispatch::new()
            .level(log::LevelFilter::Error)
            .level_for(cargo::PKG_NAME, log::LevelFilter::Info)
            .format(format_display_log)
            .chain(io::stdout())
    };

    let log_file = utils::fs::temp_file("log");
    let debug_log = fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
        .format(format_debug_log)
        .chain(fern::log_file(&log_file)?);

    fern::Dispatch::new()
        .chain(display_log)
        .chain(debug_log)
        .apply()
        .context("Failed to init the logger")?;
    Ok(log_file)
}

fn format_display_log(
    out: FormatCallback,
    message: &Arguments,
    record: &Record,
) {
    let message = match record.level() {
        log::Level::Error => message.to_string().red().bold(),
        log::Level::Warn => message.to_string().yellow(),
        log::Level::Info => message.to_string().green(),
        _ => ColoredString::from(&message.to_string() as &str),
    };
    out.finish(format_args!("{}", message))
}

fn format_debug_log(out: FormatCallback, message: &Arguments, record: &Record) {
    out.finish(format_args!(
        "[{}] [{}] {}",
        Local::now().format("%d-%m-%Y %H:%M:%S"),
        record.level(),
        message
    ))
}
