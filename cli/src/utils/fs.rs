use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Ok, Result};
use chrono::Local;
use uuid::Uuid;

use super::cargo;

pub fn find_item_by_extension<P: AsRef<Path>>(
    directory: P,
    extension: &str,
) -> Result<PathBuf> {
    for item in fs::read_dir(&directory)? {
        let path = item?.path();
        if let Some(ext) = path.extension() {
            if ext.eq(extension) {
                return Ok(path);
            }
        }
    }
    Err(anyhow!(format!(
        "*.{} not found in {}",
        extension,
        directory.as_ref().display()
    )))
}

pub fn copy_dir<S, D>(src: S, dst: D) -> Result<PathBuf>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    let (src, dst) = (src.as_ref(), dst.as_ref());
    assert!(
        src.is_dir() && src.try_exists()?,
        "{} is a existing directory",
        dst.display()
    );

    if !dst.try_exists()? {
        fs::create_dir_all(dst)?;
    }
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::dir::copy(src, dst, &options)
        .map(|_| ())
        .with_context(|| {
            format!("Failed to copy {} to {}", src.display(), dst.display())
        })?;
    Ok(dst.join(src.file_name().unwrap()))
}

pub fn copy_file<S, D>(src: S, dst: D) -> Result<PathBuf>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
{
    let (src, dst) = (src.as_ref(), dst.as_ref());
    assert!(src.is_file(), "{} is a file", src.display());
    let parent = if dst.is_file() {
        dst.parent()
    } else {
        Some(dst)
    };
    if let Some(parent) = parent {
        if !parent.try_exists()? {
            fs::create_dir_all(parent)?;
        }
    }
    let path = if dst.is_file() {
        PathBuf::from(dst)
    } else {
        dst.join(src.file_name().unwrap())
    };

    fs::copy(src, &path).map(|_| ()).with_context(|| {
        format!("Failed to copy {} to {}", src.display(), path.display())
    })?;

    Ok(path)
}

pub fn temp_file(extension: &str) -> PathBuf {
    env::temp_dir().join(format!("{}.{}", unique_name(), extension))
}

pub fn temp_dir() -> PathBuf {
    temp_dir_in(env::temp_dir())
}

pub fn temp_dir_in<P: AsRef<Path>>(directory: P) -> PathBuf {
    directory.as_ref().join(unique_name())
}

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    fs::create_dir_all(&path).with_context(|| {
        format!("Failed to create a directory: {}", path.as_ref().display())
    })
}

fn unique_name() -> String {
    format!(
        "{}-{}-{}",
        cargo::PKG_NAME,
        Local::now().format("%Y-%m-%d-%H-%M-%S"),
        Uuid::new_v4(),
    )
}
