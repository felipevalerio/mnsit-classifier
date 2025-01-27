use image::{DynamicImage, GenericImageView};


fn main() {

    let img = image::open("C:/Users/Pichau/Documents/mnist_png/training/6/13.png").unwrap();
    let gray_img = img.to_luma8();
    let pixels = gray_img.pixels();

    for pix in pixels {
        println!("{:?}", pix)
    }
}
