const GRADIENT_LEN: usize = 1 << 16;
const TABLE_SIZE: usize = 128;
const RES_X: usize = 2048;
const RES_Y: usize = 1556;

fn main() {
    // Build the test image.
    let pixels = {
        let mut pixels = vec![[0.0f32; 3]; RES_X * RES_Y];
        assert!(((GRADIENT_LEN * 4) + (TABLE_SIZE * TABLE_SIZE * TABLE_SIZE)) <= pixels.len());

        // Keep track of which pixel we're at.
        let mut pi = 0;

        // Gray gradient.
        for i in 0..GRADIENT_LEN {
            let v = i as f32 / (GRADIENT_LEN - 1) as f32;
            pixels[pi] = [v, v, v];
            pi += 1;
        }

        // Red gradient.
        for i in 0..GRADIENT_LEN {
            let v = i as f32 / (GRADIENT_LEN - 1) as f32;
            pixels[pi][0] = v;
            pi += 1;
        }

        // Green gradient.
        for i in 0..GRADIENT_LEN {
            let v = i as f32 / (GRADIENT_LEN - 1) as f32;
            pixels[pi][1] = v;
            pi += 1;
        }

        // Blue gradient.
        for i in 0..GRADIENT_LEN {
            let v = i as f32 / (GRADIENT_LEN - 1) as f32;
            pixels[pi][2] = v;
            pi += 1;
        }

        // 3D RGB table.
        for r in 0..TABLE_SIZE {
            let rn = r as f32 / (TABLE_SIZE - 1) as f32;
            for g in 0..TABLE_SIZE {
                let gn = g as f32 / (TABLE_SIZE - 1) as f32;
                for b in 0..TABLE_SIZE {
                    let bn = b as f32 / (TABLE_SIZE - 1) as f32;
                    pixels[pi] = [rn, gn, bn];
                    pi += 1;
                }
            }
        }

        pixels
    };

    // Write the test image.
    use exr::{
        image::{Encoding, Image, Layer, SpecificChannels},
        meta::header::LayerAttributes,
        prelude::WritableImage,
    };
    Image::from_layer(Layer::new(
        (1920, 1080),
        LayerAttributes::named(""),
        Encoding::SMALL_LOSSLESS,
        SpecificChannels::rgb(|co: exr::math::Vec2<usize>| {
            let rgb = pixels[co.1 * RES_X + co.0];
            (rgb[0], rgb[1], rgb[2])
        }),
    ))
    .write()
    .to_file("lut_extractor_2048x1556.exr")
    .unwrap();
}
