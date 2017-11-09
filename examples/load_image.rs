
extern crate image;

use image::GenericImage;

use std::path::Path;

fn main() {
    let file = "textures/01-brickwall.jpg";
    let im = image::open(&Path::new(&file)).unwrap();

    println!("dimensions {:?}", im.dimensions())
}
