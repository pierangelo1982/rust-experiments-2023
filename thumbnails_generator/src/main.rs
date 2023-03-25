use image::GenericImageView;
use std::fs;
use std::path::Path;

fn main() {

    println!("thumbnail generator");
    // read files in folder
    for file in fs::read_dir("images/originals").unwrap() {
        let img_path: String = file.unwrap().path().display().to_string();
        println!("show me the path: {}", img_path);
        thumbnail_generator("images/originals/image_1.jpg");
        //println!("{}", file.unwrap().path().display());
    }
   }


fn thumbnail_generator(image_path: &str) {
    println!("percordo immagine {:?}", image_path);
     let img = image::open(image_path).unwrap();
    
     //let filename: str = image_path.;
     let filename = Path::new(& image_path).file_name();
     println!("File name is {:?}", filename);
    
    println!("dimensions {:?}", img.dimensions());
   
    let width:u32 = img.width();
    let height:u32 = img.height();

    println!("width: {:?}", width);
    println!("height: {:?}", height);

    //img.resize(300, 300, image::imageops::FilterType::Gaussian);
    let thumb  = img.thumbnail(width/10, height/10);

    let filename_tmp: String = format!("{:?}", filename).to_string();
    let mut save_thumbnail_path: String=  "images/thumbnails/".to_owned();
    let borrowed_string: String = save_thumbnail_path + filename_tmp;
    
    thumb.save(borrowed_string).unwrap();


}
