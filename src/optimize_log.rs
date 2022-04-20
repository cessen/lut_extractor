use crate::linear_log::log_to_linear as log_to_lin;

pub fn find_parameters(lut: &[f32]) {
    let lin_norm = 1.0 / (lut.len() - 1) as f64;

    // Compute the stuff that we can without estimation.
    let offset = lut[0] as f64;
    let slope = lin_norm / (lut[1] as f64 - lut[0] as f64);

    // Select a range of points from the lookup table to fit to.
    let idxs: Vec<_> = (0..lut.len()).step_by(lut.len() / 256).collect();
    let coords: Vec<(f64, f64)> = idxs
        .iter()
        .map(|i| (*i as f64 * lin_norm, lut[*i] as f64))
        .collect();

    // Do the fitting.
    let f = |v: &[f64]| {
        let mut avg_sqr_err = 0.0f64;
        for (x, y) in coords.iter().copied() {
            let e = (log_to_lin(x, offset, slope, v[0], v[1]) - y).abs() / y.abs();
            avg_sqr_err += e * e;
        }
        let last_y = lut[lut.len() - 1] as f64;
        let e = (log_to_lin(1.0, offset, slope, v[0], v[1]) - last_y).abs() / last_y.abs();
        avg_sqr_err += e * e;
        avg_sqr_err
    };
    let input_interval = vec![(-0.2, 0.2), (1.1, 1000.0)];
    let (_, params) = simplers_optimization::Optimizer::minimize(&f, &input_interval, 1000000);

    // Calculate the error of our model.
    let mut max_err = 0.0f64;
    let mut avg_err = 0.0f64;
    let mut avg_samples = 0usize;
    for (i, y) in lut.iter().map(|y| *y as f64).enumerate() {
        // We only record error for values that aren't crazy tiny, since
        // their relative error isn't representative.
        if y.abs() > 0.0001 {
            let x = i as f64 * lin_norm;
            let e = (log_to_lin(x, offset, slope, params[0], params[1]) - y).abs()
            / y.abs();
            max_err = max_err.max(e);
            avg_err += e;
            avg_samples += 1;
        }
    }
    avg_err /= avg_samples as f64;

    println!("Max Err: {:.4}%\nAvg Err: {:.4}%", max_err * 100.0, avg_err * 100.0);

    println!(
        "{}{}",
        crate::linear_log::generate_linear_to_log(offset, slope, params[0], params[1],),
        crate::linear_log::generate_log_to_linear(offset, slope, params[0], params[1],),
    );
}
