use nannou::prelude::*;

pub fn _bezier_point(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    // https://github.com/processing/p5.js/blob/37484907defe7d424172b97adde966ffa8d4396b/src/core/shape/curves.js#L174
    let adjusted_t = 1.0 - t;
    pow(adjusted_t, 3) * a +
    3.0 * pow(adjusted_t, 2) * t * b +
    3.0 * adjusted_t * pow(t, 2) * c +
    pow(t, 3) * d
}

pub fn _show_bez(draw: &Draw, pos: Vec2, size: f32, color: Rgb<u8>){
    draw
        .ellipse()
        .xy(pos)
        .radius(size)
        .color(color);
}

pub fn _norm_range(f: f32) -> f32 {
    if -1.0 <= f && f <= 1.0 {
        (f + 1.0) * 0.5
    } else {
        panic!("Expected a value between -1.0 and 1.0: {:?}", f);
    }
}

// TODO: Add set_alpha(color: Srgb, alpha: f32) -> Srgba