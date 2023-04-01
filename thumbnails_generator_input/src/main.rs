use image::GenericImageView;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("thumbnail generator");

    let stdin = io::stdin();

    let mut image_original_folder_path_input = String::new();
    println!("inserisci il percorso della cartella immagini:");
    //stdin.read_line(&mut image_original_folder_path_input);
    stdin.read_line(&mut image_original_folder_path_input);
    let mut image_thumbnails_folder_path_input = String::new();
    println!("inserisci il percorso della cartella dove salvare le thumbnails:");
    stdin.read_line(&mut image_thumbnails_folder_path_input);

    println!(
        "elaboro le immagini nel percorso: {} ",
        image_original_folder_path_input
    );
    println!(
        "salvo le thumbnails nel percorso: {} ",
        image_thumbnails_folder_path_input
    );

    // read files in folder
    let origin_path = &image_original_folder_path_input.trim();
    let thumb_path = &image_thumbnails_folder_path_input.trim();
    for file in fs::read_dir(&origin_path).unwrap() {
        let img_path: String = file.unwrap().path().display().to_string();
        println!("show me the path: {}", img_path);

        thumbnail_generator(&img_path, &thumb_path);
    }

}

fn thumbnail_generator(image_path: &str, thumbnails_save_path: &str) {
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

    if width > 1200 {
        let thumb = img.thumbnail(1200, 1200);

        let save_path: String = thumbnails_save_path.to_owned();
        let new_path = save_path + filename;
        println!("percorso salvataggio thumbnails: {}", new_path);

        thumb.save(new_path).unwrap();
    } else {
        println!("inferiore")
    }
}
