use clap::{arg, Parser};
use image::ImageFormat;
use std::fs::{self, DirEntry};
use std::path::Path;

#[derive(Parser)]
#[command(name = "Crop Img")]
#[command(version = "0.1")]
#[command(about = "Crops imgs", long_about = None)]
struct Args {
    #[arg(long = "src", short = 's', required = true)]
    src_path: String,
    #[arg(long = "dest", short = 'd', required = true)]
    dest_path: String,
    #[arg(long = "dest_format", default_value = "png")]
    format: String,
    #[arg(long = "dest_size", default_value_t = 64)]
    size: u32,
}

fn crop_and_write_img(src_path: &Path, dest_path: &Path, size: u32, img_format: ImageFormat) {
    let img = image::open(src_path).expect("read src_path");
    let mid_width = img.width().div_ceil(2) - size;
    let mid_height = img.height().div_ceil(2) - size;

    let cropped_img = img.crop_imm(mid_width, mid_height, size, size);

    cropped_img
        .save_with_format(dest_path, img_format)
        .expect("save cropped image");
}

fn main() {
    let args = Args::parse();

    assert!(
        ImageFormat::from_extension(&args.format).is_some(),
        "{} image format is not supported",
        &args.format
    );
    let format = args.format;

    let src_path = Path::new(&args.src_path);
    assert!(
        src_path.is_dir(),
        "{} does not exist or is not a folder",
        src_path.to_str().expect("path to be UTF-8")
    );

    let dest_path = Path::new(&args.dest_path);
    assert!(
        dest_path.is_dir(),
        "{} does not exist or is not a folder",
        dest_path.to_str().expect("path to be UTF-8")
    );

    let src_dir = fs::read_dir(&args.src_path).expect("reading src_dir succeeds");

    let img_dir_entries = src_dir
        .filter_map(Result::ok)
        .filter(|entry| ImageFormat::from_path(entry.path().as_path()).is_ok())
        .collect::<Vec<DirEntry>>();
    assert!(
        !img_dir_entries.is_empty(),
        "src_path folder does not have image files"
    );

    for entry in img_dir_entries.iter() {
        let src_path = entry.path();
        let file_name = entry.file_name();

        let mut dest_path_buffer = dest_path.to_path_buf();
        dest_path_buffer.push(file_name);
        dest_path_buffer.set_extension(&format);

        crop_and_write_img(
            src_path.as_path(),
            dest_path_buffer.as_path(),
            args.size,
            ImageFormat::from_extension(&format).unwrap(),
        );
    }
}
