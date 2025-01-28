use image::{DynamicImage, GenericImageView};


fn main() {

    let img = image::open("C:/Users/Pichau/Documents/mnist_png/training/6/13.png").unwrap();
    let gray_img = img.to_luma8();

    // Obtém os pixels da imagem
    let pixels: Vec<f32> = gray_img.pixels()
        .map(|pix| pix[0] as f32 / 255.0) // Normaliza cada pixel para [0, 1]
        .collect(); // Coleta os resultados em um Vec<f32>

    // Exibe os valores normalizados
    for (i, &pixel) in pixels.iter().enumerate() {
        println!("Pixel {}: {}", i, pixel);
    }
}


