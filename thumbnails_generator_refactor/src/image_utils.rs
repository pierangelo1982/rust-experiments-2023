use image::GenericImageView;
use std::path::Path;

pub(crate) fn thumbnail_generator(image_path: &str) {
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

    let thumb = img.thumbnail(300, 300);

    let save_path: String = "images/thumbnails/".to_owned();
    let new_path = save_path + filename;
    println!("percorso salvataggio thumbnails: {}", new_path);

    thumb.save(new_path).unwrap();
}
