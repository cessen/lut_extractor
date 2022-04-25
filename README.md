# LUT Extractor

This is a relatively simple command line tool to help get transfer function LUTs and color space chromaticities out of other color processing tools, as long as they support reading and writing OpenEXR image files.


## Extracting transfer functions.

First, generate the test image:

```sh
lut_extractor --test_image
```

Load the resulting `lut_extractor_2560x1440.exr` image into your color processing software and run it through the processing that would convert *from* encoded color *to* linear color, and save the processed image as a separate OpenEXR image.

Important notes:

- The new image must be *exactly the same resolution* as the test image.  Any image scaling, blurring, etc. will break this process.
- Ensure that *no other color processing* is being done other than linearizing the RGB values.  For example, ensure that no gamut conversion is being done at the same time.
- Ideally, you should save it as a 32-bit floating point, lossless OpenEXR image.  Half float will reduce precision, and lossy compression can cause artifacts in the computed LUT.

Next, run the processed OpenEXR image through lut_extractor like this:

```sh
lut_extractor -i filename.exr
```

This will produce two LUT files, one in `.cube` format and one in `.spi1d` format.

It also attempts an analytic fit to the LUT, but this only works well for specific kinds of transfer functions.  Error statistics and pseudo code of the fit are printed. (Note: Max Relative Error is the most relevant statistic, and should be below 0.01 for an okay fit and below 0.001 for a good fit.)

And that's it!


## Extracting chromaticity coordinates.

This is essentially the same process as above, except doing a color gamut conversion instead of a linearizing conversion:

Process the test image into CIE XYZ color space, and save it as a separate OpenEXR image.  Ensure that *only* gamut conversion is being done.  For example, no non-linear -> linear conversion.

Next, run the processed OpenEXR image through lut_extractor like this:

```sh
lut_extractor --chroma filename.exr
```

It will then print the chromaticity coordinates.


## License

This software is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
