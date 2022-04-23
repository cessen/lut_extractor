pub const GRADIENT_LEN: usize = 1 << 17;
pub const TABLE_SIZE: usize = 144;
pub const RES_X: usize = 2560;
pub const RES_Y: usize = 1440;

// Compile time assert.
const _: () = {
    assert!(((GRADIENT_LEN * 4) + (TABLE_SIZE * TABLE_SIZE * TABLE_SIZE)) <= (RES_X * RES_Y));
};

/// Creates the test image, as a Vec of RGB triples.
pub fn build() -> Vec<[f32; 3]> {
    let mut pixels = vec![[0.0f32; 3]; RES_X * RES_Y];

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
}

/// Gives the index in the test image Vec of the given
/// gray-gradient index.
#[allow(unused)]
#[inline(always)]
pub fn gray_idx(idx: usize) -> usize {
    idx
}

/// Gives the index in the test image Vec of the given
/// red-gradient index.
#[allow(unused)]
#[inline(always)]
pub fn red_idx(idx: usize) -> usize {
    GRADIENT_LEN + idx
}

/// Gives the index in the test image Vec of the given
/// green-gradient index.
#[allow(unused)]
#[inline(always)]
pub fn green_idx(idx: usize) -> usize {
    GRADIENT_LEN * 2 + idx
}

/// Gives the index in the test image Vec of the given
/// blue-gradient index.
#[allow(unused)]
#[inline(always)]
pub fn blue_idx(idx: usize) -> usize {
    GRADIENT_LEN * 3 + idx
}

/// Gives the index in the test image Vec of the given
/// rgb table indices.
#[allow(unused)]
#[inline(always)]
pub fn rgb_idx(r_idx: usize, g_idx: usize, b_idx: usize) -> usize {
    (GRADIENT_LEN * 4) + (r_idx * TABLE_SIZE * TABLE_SIZE) + (g_idx * TABLE_SIZE) + b_idx
}
