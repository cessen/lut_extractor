mod linear_log;
mod optimize_log;
mod test_image;

use std::{fs::File, io::BufWriter, path::Path};

use clap::{Arg, Command};
// use colorbox::transfer_functions::srgb;

use test_image::{GRADIENT_LEN, RES_X, RES_Y};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LUT_LEN: usize = 1 << 12;

fn main() {
    let args = Command::new("LUT Extractor")
        .version(VERSION)
        .about("A small utility for extracting color LUTs from other software.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("A processed test EXR file, to extract a LUT.")
                .takes_value(true)
                .required_unless_present_any(&["test_image"]),
        )
        .arg(
            Arg::new("test_image")
                .short('t')
                .long("test_image")
                .help("Generates the test EXR file."),
        )
        .get_matches();

    if args.is_present("test_image") {
        // Build the test image.
        let pixels = test_image::build();

        // Write the test image.
        write_rgb_exr(
            &format!("lut_extractor_{}x{}.exr", RES_X, RES_Y),
            &pixels,
            RES_X,
            RES_Y,
        )
        .unwrap();
    } else {
        let input_path = args.value_of("input").unwrap();

        // let base_image = test_image::build();
        let mut input_image = {
            let (image, res_x, res_y) = read_rgb_exr(input_path).unwrap();
            assert_eq!(
                res_x, RES_X,
                "Input image doesn't have the correct resolution."
            );
            assert_eq!(
                res_y, RES_Y,
                "Input image doesn't have the correct resolution."
            );
            image
        };

        // Fetch the transfer function LUT.
        let gray = &mut input_image[test_image::gray_idx(0)..test_image::gray_idx(GRADIENT_LEN)];

        // Attempt to find an analytic log-linear function that matches
        // the transfer function.
        let full_lut: Vec<f32> = gray
            .iter()
            .map(|rgb| ((rgb[0] as f64 + rgb[1] as f64 + rgb[2] as f64) / 3.0) as f32)
            .collect();
        optimize_log::find_parameters(&full_lut);

        // Build the LUT for export.
        let mut prev = gray[0];
        for rgb in gray.iter_mut() {
            // Ensure montonicity.
            if rgb[0] < prev[0] {
                rgb[0] = prev[0];
            }
            if rgb[1] < prev[1] {
                rgb[1] = prev[1];
            }
            if rgb[2] < prev[2] {
                rgb[2] = prev[2];
            }
            prev = *rgb;
        }
        let mut gray_r = Vec::with_capacity(LUT_LEN);
        let mut gray_g = Vec::with_capacity(LUT_LEN);
        let mut gray_b = Vec::with_capacity(LUT_LEN);
        for i in 0..LUT_LEN {
            let t = i as f32 / (LUT_LEN - 1) as f32;
            let rgb = lerp_slice(gray, t);
            gray_r.push(rgb[0]);
            gray_g.push(rgb[1]);
            gray_b.push(rgb[2]);
        }

        // Write the LUT.
        colorbox::formats::cube::write_1d(
            BufWriter::new(File::create("test.cube").unwrap()),
            [(0.0, 1.0); 3],
            [&gray_r, &gray_g, &gray_b],
        )
        .unwrap();
    }
}

fn lerp_slice(slice: &[[f32; 3]], t: f32) -> [f32; 3] {
    assert!(!slice.is_empty());

    let t2 = (slice.len() - 1) as f32 * t;
    let t2i = t2 as usize;

    if t2i == (slice.len() - 1) {
        *slice.last().unwrap()
    } else {
        let alpha = t2.fract();
        let inv_alpha = 1.0 - alpha;
        let a = slice[t2i];
        let b = slice[t2i + 1];
        [
            (a[0] * inv_alpha) + (b[0] * alpha),
            (a[1] * inv_alpha) + (b[1] * alpha),
            (a[2] * inv_alpha) + (b[2] * alpha),
        ]
    }
}

fn read_rgb_exr<P: AsRef<Path>>(path: P) -> exr::error::Result<(Vec<[f32; 3]>, usize, usize)> {
    use exr::prelude::{ReadChannels, ReadLayers};

    let image = exr::image::read::read()
        .no_deep_data()
        .largest_resolution_level()
        .rgb_channels(
            |res, _| (vec![[0.0f32; 3]; res.0 * res.1], res.0, res.1),
            |(pixels, res_x, _res_y), co, (r, g, b): (f32, f32, f32)| {
                pixels[co.1 * *res_x + co.0] = [r, g, b];
            },
        )
        .first_valid_layer()
        .all_attributes()
        .from_file(path)?;

    Ok(image.layer_data.channel_data.pixels)
}

fn write_rgb_exr<P: AsRef<Path>>(
    path: P,
    pixels: &[[f32; 3]],
    res_x: usize,
    res_y: usize,
) -> std::io::Result<()> {
    use exr::{
        image::{Encoding, Image, Layer, SpecificChannels},
        meta::header::LayerAttributes,
        prelude::WritableImage,
    };

    assert_eq!(pixels.len(), res_x * res_y);

    match Image::from_layer(Layer::new(
        (res_x, res_y),
        LayerAttributes::named(""),
        Encoding::SMALL_LOSSLESS,
        SpecificChannels::rgb(|co: exr::math::Vec2<usize>| {
            let rgb = pixels[co.1 * res_y + co.0];
            (rgb[0], rgb[1], rgb[2])
        }),
    ))
    .write()
    .to_file(path)
    {
        // Only IO errors should be possible here.
        Err(exr::error::Error::Io(e)) => Err(e),
        Err(_) => panic!(),
        Ok(()) => Ok(()),
    }
}
