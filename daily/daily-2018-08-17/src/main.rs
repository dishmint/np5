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
    max_w: f32,
    box_w: f32,
    box_h: f32,
    mod_n: u64,
}

fn model(app: &App) -> Model {
    let rate  = 10.0;
    let max_w = app.window_rect().w() * 0.5;

    let box_w = 0.0;
    let box_h = 0.0;

    let mod_n = 5;

    Model { rate, max_w, box_w, box_h, mod_n }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time * model.rate;
    
    let cos_x = time.cos();
    let sin_y = time.sin();
    
    if app.elapsed_frames() % model.mod_n == 0 {
        model.box_w = cos_x * model.max_w;
        model.box_h = sin_y * model.max_w;
    }
    
    // let cos_x = time.cos();
    // let sin_y = time.sin();
    
    // model.box_w = cos_x * model.max_w;
    // model.box_h = sin_y * model.max_w;

    if model.box_w > model.max_w {
        model.box_w = 0.0;
        model.box_h = 0.0;
    }

}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    draw.rect()
        .color(LIME)
        .w_h(model.box_w, model.box_h);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.098));
    draw.to_frame(app, &frame).unwrap();
}