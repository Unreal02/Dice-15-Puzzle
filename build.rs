use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{imageops, ImageBuffer, Rgb, RgbImage};

const BORDER_WIDTH: u32 = 5;

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

    // put in side faces
    let mut img_goal_small = imageops::resize(&img_goal, 512, 512, FilterType::Nearest);
    for x in 0..512 {
        for y in 0..512 {
            let mut pixel = *img_goal_small.get_pixel(x, y);
            for i in 0..3 {
                pixel.0[i] = ((pixel.0[i] as u32 + 128) * 2 / 3) as u8;
            }
            img_goal_small.put_pixel(x, y, pixel);
        }
    }

    // output image
    let mut img: RgbImage = ImageBuffer::new(1024, 768);
    for x in 0..1024 {
        for y in 0..768 {
            img.put_pixel(x, y, Rgb::from([255, 255, 255]));
        }
    }

    // sides
    for x in 0..256 {
        for y in 0..256 {
            let side_pixel = *img_side.get_pixel(x, y);
            // up
            img.put_pixel(511 - x, 255 - y, side_pixel);
            // down
            img.put_pixel(256 + x, 512 + y, side_pixel);
            // left
            img.put_pixel(256 - y, 256 + x, side_pixel);
            // right
            img.put_pixel(512 + y, 511 - x, side_pixel);
            // back
            img.put_pixel(768 + x, 256 + y, *img_back.get_pixel(x, y));
        }
    }

    // border
    for i in 0..BORDER_WIDTH {
        let black = Rgb::from([32, 32, 32]);
        for x in 0..1024 {
            img.put_pixel(x, i, black);
            img.put_pixel(x, 255 - i, black);
            img.put_pixel(x, 256 + i, black);
            img.put_pixel(x, 511 - i, black);
            img.put_pixel(x, 512 + i, black);
            img.put_pixel(x, 767 - i, black);
        }
        for y in 0..768 {
            img.put_pixel(i, y, black);
            img.put_pixel(255 - i, y, black);
            img.put_pixel(256 + i, y, black);
            img.put_pixel(511 - i, y, black);
            img.put_pixel(512 + i, y, black);
            img.put_pixel(767 - i, y, black);
            img.put_pixel(768 + i, y, black);
            img.put_pixel(1023 - i, y, black);
        }
    }

    for i in 0..15 {
        for x in 0..256 {
            for y in 0..256 {
                let goal_pixel = *img_goal.get_pixel(x + (i % 4) * 256, y + (i / 4) * 256);
                // goal
                if x >= BORDER_WIDTH
                    && x < 256 - BORDER_WIDTH
                    && y >= BORDER_WIDTH
                    && y < 256 - BORDER_WIDTH
                {
                    img.put_pixel(x + 256, y + 256, goal_pixel);
                }
            }
        }
        for x in 0..128 {
            for y in 0..128 {
                let goal_small_pixel =
                    *img_goal_small.get_pixel(x + (i % 4) * 128, y + (i / 4) * 128);
                // up
                img.put_pixel(256 + 64 + x, 64 + y, goal_small_pixel);
                // down
                img.put_pixel(256 + 64 + x, 512 + 64 + y, goal_small_pixel);
                // left
                img.put_pixel(64 + x, 256 + 64 + y, goal_small_pixel);
                // right
                img.put_pixel(512 + 64 + x, 256 + 64 + y, goal_small_pixel);
                // back
                img.put_pixel(768 + 64 + x, 256 + 64 + y, goal_small_pixel);
            }
        }
        img.save(format!("assets/images/image{}.png", i + 1))
            .unwrap();
    }
}
