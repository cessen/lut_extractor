mod test_image;

use test_image::{RES_X, RES_Y};

fn main() {
    // Build the test image.
    let pixels = test_image::build();

    // Write the test image.
    use exr::{
        image::{Encoding, Image, Layer, SpecificChannels},
        meta::header::LayerAttributes,
        prelude::WritableImage,
    };
    Image::from_layer(Layer::new(
        (RES_X, RES_Y),
        LayerAttributes::named(""),
        Encoding::SMALL_LOSSLESS,
        SpecificChannels::rgb(|co: exr::math::Vec2<usize>| {
            let rgb = pixels[co.1 * RES_X + co.0];
            (rgb[0], rgb[1], rgb[2])
        }),
    ))
    .write()
    .to_file(format!("lut_extractor_{}x{}.exr", RES_X, RES_Y))
    .unwrap();
}
