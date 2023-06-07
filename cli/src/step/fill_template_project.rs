use std::{
    cmp::max,
    cmp::min,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Ok, Result};
use clap::ValueEnum;
use image::{
    codecs::gif::{self, GifEncoder},
    imageops, AnimationDecoder, DynamicImage, Frame, ImageDecoder, RgbaImage,
};
use log::{debug, info};

use crate::utils;

pub fn fill_template_project<T: AsRef<Path>, I: AsRef<Path>>(
    template: T,
    command: &str,
    icon_src: I,
    resize_icon_method: ResizeMethod,
) -> Result<()> {
    info!("Filling the template...");
    write_command_file(&template, command)?;
    write_app_icon(&template, &icon_src, resize_icon_method)?;
    write_live_app_icon(&template, &icon_src, resize_icon_method)?;
    Ok(())
}

fn write_command_file<T: AsRef<Path>>(
    template: T,
    command: &str,
) -> Result<()> {
    let mut path = PathBuf::from(template.as_ref());
    path.push("LiveAppIcon");
    path.push("Assets.xcassets");
    path.push("Command.dataset");
    path.push("command.txt");
    debug!("Writing the command file to {}", path.display());
    let mut file = File::create(path)?;
    file.write_all(command.as_bytes())?;
    Ok(())
}

fn write_live_app_icon<T: AsRef<Path>, I: AsRef<Path>>(
    template: T,
    icon: I,
    resize_method: ResizeMethod,
) -> Result<()> {
    let mut output_path = PathBuf::from(template.as_ref());
    output_path.push("LiveAppIcon");
    output_path.push("Assets.xcassets");
    output_path.push("LiveAppIcon.dataset");
    output_path.push("live_app_icon.gif");

    debug!("Creating the animated icon to {}", output_path.display());

    let decoder = gif::GifDecoder::new(File::open(&icon)?)?;
    let (width, height) = decoder.dimensions();
    let should_resize = width != height;

    if !should_resize {
        utils::fs::copy_file(icon, &output_path)?;
        return Ok(());
    }

    debug!("Resizing the given GIF...");
    let squared_frames =
        decoder.into_frames().enumerate().map(|(index, frame)| {
            debug!("Resizing frame {}", index + 1);
            frame.map(|frame| {
                let delay = frame.delay();
                Frame::from_parts(
                    resize_image_to_square(
                        &DynamicImage::ImageRgba8(frame.into_buffer()),
                        resize_method,
                        None,
                    )
                    .into_rgba8(),
                    0,
                    0,
                    delay,
                )
            })
        });

    let mut encoder = GifEncoder::new(File::create(&output_path)?);
    encoder.set_repeat(gif::Repeat::Infinite)?;
    encoder.try_encode_frames(squared_frames)?;
    Ok(())
}

fn write_app_icon<T: AsRef<Path>, I: AsRef<Path>>(
    template: T,
    icon: I,
    resize_method: ResizeMethod,
) -> Result<()> {
    let decoder = gif::GifDecoder::new(File::open(icon)?)?;
    let first_frame = DynamicImage::ImageRgba8(
        decoder.into_frames().next().unwrap()?.into_buffer(),
    );

    // 16px, 32px, ..., 1024px
    for icon_size in (4..=10).map(|i| 2u32.pow(i)) {
        let mut path = PathBuf::from(template.as_ref());
        path.push("LiveAppIcon");
        path.push("Assets.xcassets");
        path.push("AppIcon.appiconset");
        path.push(format!("{}.png", icon_size));
        debug!(
            "Creating a static app icon of size {}px to {}",
            icon_size,
            path.display()
        );
        resize_image_to_square(&first_frame, resize_method, Some(icon_size))
            .save(&path)
            .with_context(|| {
                format!("Failed to save an app icon to {}", path.display())
            })?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ResizeMethod {
    CenterCrop,
    CenterFit,
}

fn resize_image_to_square(
    image: &DynamicImage,
    method: ResizeMethod,
    expected_size: Option<u32>,
) -> DynamicImage {
    let (width, height) = (image.width(), image.height());
    let squared_image = match method {
        ResizeMethod::CenterCrop => {
            let crop_size = min(width, height);
            image.crop_imm(
                (width - crop_size) / 2,
                (height - crop_size) / 2,
                crop_size,
                crop_size,
            )
        }
        ResizeMethod::CenterFit => {
            let new_size = max(width, height);
            let mut new_image = RgbaImage::new(new_size, new_size);
            new_image.fill(0); // Transparent background
            let h_margin = ((new_size - width) / 2) as i64;
            let v_margin = ((new_size - height) / 2) as i64;
            imageops::overlay(&mut new_image, image, h_margin, v_margin);
            DynamicImage::ImageRgba8(new_image)
        }
    };

    match expected_size {
        Some(expected_size) => squared_image.resize_exact(
            expected_size,
            expected_size,
            imageops::FilterType::Gaussian,
        ),
        None => squared_image,
    }
}
