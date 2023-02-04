mod ascii;
mod autotrait;
mod broom;
mod generics;
mod grayscale;
mod imple;
mod inner;
mod lifetime;
mod tuplestruct;
mod vector;

fn main() {
    let width = 1024;
    let height = 576;
    let image = grayscale::new_map((width, height), vec![0; width * height]);

    println!("image # of pixels: {}", image.pixels.len());
    println!("image width: {}, height: {}", image.size.0, image.size.1);
}
