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

//-------------------------------------------------------------

/// Generates Rust code for a linear-to-log transfer function with the
/// given parameters.
pub fn generate_linear_to_log(line_offset: f64, slope: f64, log_offset: f64, base: f64) -> String {
    let transition = 1.0 / (slope * base.ln());
    let k = transition + log_offset;
    let l = (transition - line_offset + log_offset) * slope - transition.log(base);

    format!(
        r#"
pub fn linear_to_log(x: f32) -> f32 {{
    const A: f32 = {};
    const B: f32 = {};
    const C: f32 = {};
    const D: f32 = {};
    const E: f32 = {};
    const F: f32 = {};

    if x <= A {{
        (x - B) * C
    }} else {{
        (x - D).log2() * (1.0 / E) + F
    }}
}}
"#,
        k,
        line_offset,
        slope,
        log_offset,
        base.log2(),
        l,
    )
}

/// Generates Rust code for a log-to-linear transfer function with the
/// given parameters.
pub fn generate_log_to_linear(line_offset: f64, slope: f64, log_offset: f64, base: f64) -> String {
    let transition = 1.0 / (slope * base.ln());
    let k = (transition - line_offset + log_offset) * slope;
    let l = (transition - line_offset + log_offset) * slope - transition.log(base);

    format!(
        r#"
pub fn log_to_linear(x: f32) -> f32 {{
    const A: f32 = {};
    const B: f32 = {};
    const C: f32 = {};
    const D: f32 = {};
    const E: f32 = {};
    const F: f32 = {};

    if x <= A {{
        (x * (1.0 / C)) + B
    }} else {{
        ((x - F) * E).exp2() + D
    }}
}}
"#,
        k,
        line_offset,
        slope,
        log_offset,
        base.log2(),
        l,
    )
}
