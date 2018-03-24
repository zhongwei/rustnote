extern crate qrcode;
extern crate image;

use qrcode::{EcLevel, QrCode, Version};
use qrcode::render::svg;
use image::Luma;

fn main() {
    // Encode some data into bits.
    let code = QrCode::with_version(b"01234567", Version::Micro(2), EcLevel::L).unwrap();
    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("/tmp/qrcode.png").unwrap();
    let image = code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();
    println!("{}", image);

    let string = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
}

