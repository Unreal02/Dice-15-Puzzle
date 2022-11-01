use image::io::Reader as ImageReader;
use image::{ImageBuffer, RgbImage};

fn main() {
    // generate image textures from
    // image_goal.png
    // image_side.png
    // image_back.png

    let img_goal = ImageReader::open("assets/images/image_goal.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let img_side = ImageReader::open("assets/images/image_side.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();
    let img_back = ImageReader::open("assets/images/image_back.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let mut img: RgbImage = ImageBuffer::new(1024, 768);
    for x in 0..256 {
        for y in 0..256 {
            let img_side_pixel = *img_side.get_pixel(x, y);
            // up
            img.put_pixel(511 - x, 255 - y, img_side_pixel);
            // down
            img.put_pixel(256 + x, 512 + y, img_side_pixel);
            // left
            img.put_pixel(256 - y, 256 + x, img_side_pixel);
            // right
            img.put_pixel(512 + y, 511 - x, img_side_pixel);
            // back
            img.put_pixel(768 + x, 256 + y, *img_back.get_pixel(x, y));
        }
    }

    for i in 0..15 {
        for x in 0..256 {
            for y in 0..256 {
                // goal
                img.put_pixel(
                    x + 256,
                    y + 256,
                    *img_goal.get_pixel(x + (i % 4) * 256, y + (i / 4) * 256),
                );
            }
        }
        img.save(format!("assets/images/image{}.png", i + 1))
            .unwrap();
    }
}
