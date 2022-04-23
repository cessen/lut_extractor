use crate::linear_log::log_to_linear as log_to_lin;

pub fn find_parameters(lut: &[f32]) {
    let lin_norm = 1.0 / (lut.len() - 1) as f64;

    // Compute the stuff that we can without estimation.
    let offset = lut[0] as f64;
    let end = lut[lut.len() - 1] as f64;
    let slope = {
        // We take the difference of points near zero for increased accuracy.
        let (mut i, _) = lut.iter().enumerate().find(|(_, y)| **y > 0.0).unwrap();
        if i == 0 {
            i += 1;
        }
        lin_norm / (lut[i] as f64 - lut[i - 1] as f64)
    };

    // Collect LUT points as (x, y) coordinates.
    let coords: Vec<(f64, f64)> = lut
        .iter()
        .enumerate()
        .step_by(lut.len() / 256)
        .map(|(i, y)| (i as f64 * lin_norm, *y as f64))
        .collect();

    // Do the fitting.
    let base = optimize(
        |v: f64| {
            let log_offset = crate::linear_log::find_log_offset_for_end(end, offset, slope, v);
            let mut sqr_err = 0.0f64;
            for (x, y) in coords.iter().copied() {
                let e = (log_to_lin(x, offset, slope, log_offset, v) - y).abs() / y.abs();
                sqr_err += e * e;
            }
            sqr_err
        },
        [1.5, 10000000.0],
        100,
    );
    let log_offset = crate::linear_log::find_log_offset_for_end(end, offset, slope, base);
    let transition = crate::linear_log::transition_point(offset, slope, log_offset, base);

    // Calculate the error of our model.
    let mut max_err = 0.0f64;
    let mut avg_err = 0.0f64;
    let mut avg_samples = 0usize;
    for (x, y) in coords.iter().copied() {
        // Only record error for the log part of the curve because we
        // computed the linear segment's slope and offset analytically,
        // and the relative error of points very near zero isn't reliable.
        if y > transition.0 {
            let e = (log_to_lin(x, offset, slope, log_offset, base) - y).abs() / y.abs();
            max_err = max_err.max(e);
            avg_err += e;
            avg_samples += 1;
        }
    }
    avg_err /= avg_samples as f64;

    println!(
        "Max Relative Error: {:.4}%\nAvg Relative Error: {:.4}%",
        max_err * 100.0,
        avg_err * 100.0
    );

    // dbg!(offset, log_offset, slope, base, end);

    println!(
        "{}",
        crate::linear_log::generate_code(offset, slope, log_offset, base),
    );
}

/// This finds the minimum of functions with only one minimum (i.e. has
/// no local minimums other than the global one).  It will not work
/// for functions that don't meet that criteria.
///
/// It works by progressively narrowing the search range by:
/// 1. Splitting the range into four equal segments.
/// 2. Checking the slope of each segment.
/// 3. Narrowing the range to the two adjecent segments where
///    there is a switch from negative to positive slope.
fn optimize<F: Fn(f64) -> f64>(f: F, range: [f64; 2], iterations: usize) -> f64 {
    let mut range = range;

    for _ in 0..iterations {
        const SEG_POINTS: usize = 5;
        let point = |xi| {
            let n = xi as f64 / (SEG_POINTS - 1) as f64;
            (range[0] * (1.0 - n)) + (range[1] * n)
        };
        let mut last_xi = 0;
        for i in 0..(SEG_POINTS - 1) {
            last_xi = i;
            let y1 = f(point(i));
            let y2 = f(point(i + 1));
            if (y2 - y1) >= 0.0 {
                break;
            }
        }
        let (r1, r2) = if last_xi == 0 {
            (point(0), point(1))
        } else if last_xi == (SEG_POINTS - 1) {
            (point(SEG_POINTS - 2), point(SEG_POINTS - 1))
        } else {
            (point(last_xi - 1), point(last_xi + 1))
        };
        range = [r1, r2];
    }

    (range[0] + range[1]) * 0.5
}
