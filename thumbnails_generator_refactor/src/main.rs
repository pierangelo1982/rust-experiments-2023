mod image_utils;
use image_utils::thumbnail_generator;
use std::fs;

fn main() {
    println!("thumbnail generator - refactor");
    // read files in folder
    for file in fs::read_dir("images/originals").unwrap() {
        let img_path: String = file.unwrap().path().display().to_string();
        println!("show me the path: {}", img_path);
        thumbnail_generator(&img_path);
    }
}
