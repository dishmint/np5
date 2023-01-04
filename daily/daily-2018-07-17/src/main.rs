use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    rate: f32,
    size: Vec2,
    r_delta: f32,
    w_delta: f32,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let rate = 1.0_f32;
    let size = win.wh() * 0.1;

    let r_delta = 0.0;
    let w_delta = 0.0;
    Model { rate, size, r_delta, w_delta }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time * model.rate;

    model.w_delta = (time.sin() + 1.0) * 0.5;
    model.r_delta = 1.0 - ((time.cos() + 1.0) * 0.5);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    draw.rect()
        .wh(model.size)
        .color(srgb(model.w_delta,model.w_delta,model.w_delta));
        
    draw.rect()
        .wh(model.size * 2.0)
        .stroke_weight(1.0)
        .stroke_color(srgb(1.0 - model.w_delta, 1.0 - model.w_delta, 1.0 - model.w_delta ))
        .no_fill();
        
        
    draw.rect()
        .wh(model.size * 3.0)
        .stroke_weight(1.0)
        .stroke_color(srgba(model.r_delta, 0.0, 0.0, 1.0 - model.r_delta))
        .no_fill();

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));
    draw.to_frame(app, &frame).unwrap();
}