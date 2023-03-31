use image::GenericImageView;
use std::fs;
use std::path::Path;

use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let interval = Duration::from_secs(3);
    let mut next_time = Instant::now() + interval;

    println!("thumbnail generator");

    // read files in folder
    for file in fs::read_dir("images/originals").unwrap() {
        let img_path: String = file.unwrap().path().display().to_string();
        println!("show me the path: {}", img_path);

        thumbnail_generator(&img_path);
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}

fn thumbnail_generator(image_path: &str) {
    println!("percordo immagine {:?}", image_path);
    let img = image::open(image_path).unwrap();

    let filename = Path::new(&image_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    println!("File name is {:?}", filename);

    println!("dimensions {:?}", img.dimensions());

    let width: u32 = img.width();
    let height: u32 = img.height();

    println!("width: {:?}", width);
    println!("height: {:?}", height);

    //img.resize(300, 300, image::imageops::FilterType::Gaussian);
    //let thumb = img.thumbnail(width / 10, height / 10);
    if width > 1200 {
        let thumb = img.thumbnail(1200, 1200);

        let save_path: String = "images/thumbnails/".to_owned();
        let new_path = save_path + filename;
        println!("percorso salvataggio thumbnails: {}", new_path);

        thumb.save(new_path).unwrap();
    } else {
        println!("inferiore")
    }
}
