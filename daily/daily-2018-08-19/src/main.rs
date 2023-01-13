use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    color: f32,
    alpha: f32,
    max_r: f32,
    radius: f32,
    rate: f32,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let color = 0.0;
    let alpha = 0.0;
    let max_r = win.right() * 0.75;
    let radius = 0.0;
    let rate = 0.2;
    Model { color, alpha, max_r, radius, rate }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time * model.rate;

    let cos_t = time.cos();
    let sin_t = time.cos();

    model.color = (cos_t + 1.0) * 0.5;
    model.alpha = (sin_t + 1.0) * 0.5;
    model.radius = model.max_r * cos_t;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    draw.ellipse()
        .radius(model.radius)
        .stroke_color(WHITE)
        .stroke_weight(1.0)
        .color(srgba(model.color, model.color, model.color, model.alpha));

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,10.0/255.0));
    draw.to_frame(app, &frame).unwrap();
}