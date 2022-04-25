/// A composite linear-log function.
///
/// `slope` is the slope of the linear segment.
/// `base` is the log base.
/// The offsets shift the linear and log parts of the curve along
/// the linear color axis.
pub fn linear_to_log(x: f64, line_offset: f64, slope: f64, log_offset: f64, base: f64) -> f64 {
    // Transition point between log and linear.
    let transition = 1.0 / (slope * base.ln());

    let k = transition + log_offset;
    let l = (transition - line_offset + log_offset) * slope - transition.log(base);

    if x <= k {
        (x - line_offset) * slope
    } else {
        (x - log_offset).log(base) + l
    }
}

/// A composite linear-log function.
///
/// `slope` is the slope of the linear segment.
/// `base` is the log base.
/// The offsets shift the linear and log parts of the curve along
/// the linear color axis.
pub fn log_to_linear(x: f64, line_offset: f64, slope: f64, log_offset: f64, base: f64) -> f64 {
    // Transition point between log and linear.
    let transition = 1.0 / (slope * base.ln());

    let k = (transition - line_offset + log_offset) * slope;
    let l = (transition - line_offset + log_offset) * slope - transition.log(base);

    if x <= k {
        (x / slope) + line_offset
    } else {
        base.powf(x - l) + log_offset
    }
}

// Find the `log_offset` needed to put x=end at y=1.0 in the linear_to_log function.
pub fn find_log_offset_for_end(end: f64, line_offset: f64, slope: f64, base: f64) -> f64 {
    let mut offset_up = 10.0;
    let mut offset_down = -10.0;

    for _ in 0..100 {
        let log_offset = (offset_up + offset_down) * 0.5;
        if linear_to_log(end, line_offset, slope, log_offset, base) > 1.0 {
            offset_up = log_offset;
        } else {
            offset_down = log_offset;
        }
    }

    offset_up
}

//-------------------------------------------------------------

/// Generates psuedo code for both linear-to-log and log-to-linear
/// functions with the given parameters.
pub fn generate_code(line_offset: f64, slope: f64, log_offset: f64, base: f64) -> String {
    let transition = 1.0 / (slope * base.ln());
    let k1 = transition + log_offset;
    let k2 = (k1 - line_offset) * slope;
    let l = (transition - line_offset + log_offset) * slope - transition.log(base);

    format!(
        r#"
A = {} (line slope)
B = {} (line y offset)
C = {} (log y scale)
D = {} (log x offset)
E = {} (log y offset)

linear_to_log(x) =
    if x <= {}:
        A * x + B
    else:
        C * ln(x + D) + E

log_to_linear(x) =
    if x <= {}:
        (x - B) / A
    else:
        e^((x - E) / C) - D
"#,
        slope,
        -line_offset * slope,
        1.0 / base.ln(),
        -log_offset,
        l,
        k1,
        k2,
    )
}
