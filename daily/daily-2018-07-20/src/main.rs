use nannou::prelude::*;

// mod fzn;
// use crate::fzn::bezier_point;
fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    rate: f32,
    pos_1: Point2,
    pos_2: Point2,
    wth_1: f32,
    clr_1: Srgba,
    clr_2: Srgba,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    let rate = 0.1;
    let pos_1 = vec2(0.0,0.0);
    let pos_2 = vec2(0.0,0.0);
    let wth_1 = win.right() * 0.5;
    let clr_1 = srgba(1.0,0.0,0.0,0.0);
    let clr_2 = srgba(0.0,0.0,1.0,0.0);

    Model { rate, pos_1, pos_2, wth_1, clr_1, clr_2 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let time = app.time * model.rate;
    model.wth_1 = abs((win.right() * 0.5) * time.sin());
    
    let x_bnd = win.right() - (model.wth_1 * 0.5);
    let y_bnd = win.top() - (model.wth_1 * 0.5);

    let x_delta = x_bnd * time.sin();
    let y_delta = y_bnd * time.sin();

    model.pos_1 = vec2(x_delta, 0.0);
    model.clr_1 = srgba(1.0,0.0,0.0, abs(time.sin()));
    
    model.pos_2 = vec2(0.0, y_delta);
    model.clr_2 = srgba(0.0,68.0/255.0,170.0/255.0, abs(time.sin()));
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    draw.rect()
        .xy(model.pos_1)
        .width(model.wth_1)
        .stroke_weight(1.0)
        .stroke_color(model.clr_1)
        .no_fill();

    draw.rect()
        .xy(-1.0 * model.pos_1)
        .width(model.wth_1)
        .stroke_weight(1.0)
        .stroke_color(model.clr_1)
        .no_fill();

    draw.rect()
        .xy(model.pos_2)
        .width(model.wth_1)
        .stroke_weight(1.0)
        .stroke_color(model.clr_2)
        .no_fill();

    draw.rect()
        .xy(-1.0 * model.pos_2)
        .width(model.wth_1)
        .stroke_weight(1.0)
        .stroke_color(model.clr_2)
        .no_fill();
    
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.01));

    draw.to_frame(app, &frame).unwrap();
}

