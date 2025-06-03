use clap::{arg, Parser};
use image::ImageFormat;
use std::fs::{self, DirEntry};
use std::path::Path;

#[derive(Parser)]
#[command(name = "Crop Img")]
#[command(version = "0.1")]
#[command(about = "Crops imgs", long_about = None)]
struct Args {
    #[arg(long)]
    src_path: String,
    #[arg(long)]
    dest_path: String,
    #[arg(long)]
    size: u32,
}

#[allow(dead_code)]
fn crop_and_write_img(src_path: String, dest_path: String, size: u32) {
    let img = image::open(src_path).unwrap();
    let mid_width = img.width().div_ceil(2) - size;
    let mid_height = img.height().div_ceil(2) - size;

    let cropped_img = img.crop_imm(mid_width, mid_height, size, size);

    println!("{mid_width} {mid_height}");

    cropped_img
        .save_with_format(dest_path, image::ImageFormat::Png)
        .unwrap();
}

fn main() {
    let args = Args::parse();

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

    let img_files: Vec<DirEntry> = src_dir
        .filter_map(Result::ok)
        .filter(|entry| ImageFormat::from_path(entry.path().as_path()).is_ok())
        .collect();

    assert!(
        !img_files.is_empty(),
        "src_path folder does not have image files"
    );
}
