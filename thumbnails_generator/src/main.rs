use image::GenericImageView;

fn main() {

    println!("Hello, world!");
    let img = image::open("images/originals/image_1.jpg").unwrap();
    
    println!("dimensions {:?}", img.dimensions());
   
    let width:u32 = img.width();
    let height:u32 = img.height();

    println!("width: {:?}", width);
    println!("height: {:?}", height);

    //img.resize(300, 300, image::imageops::FilterType::Gaussian);
    let thumb  = img.thumbnail(width/10, height/10);
    thumb.save("images/thumbnails/image_1.jpg").unwrap();
}
