use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    change: f32,
    hw: f32
}

fn model(app: &App) -> Model {
    let change = 0.0;
    let hw = app.window_rect().w() * 0.5;

    Model {change, hw}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let tri = app.time
        .sin()
        .asin() * (4.0 / TAU);
    
    model.change = tri * model.hw;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    let _time = app.time;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    draw.ellipse()
        .w_h(model.change, model.hw)
        // .color(TOMATO);
        .color(MEDIUMSEAGREEN);
    
    /* 
    draw.ellipse()
        .w_h(-model.change * 0.125, model.hw * 0.125)
        .color(BLACK);
    
    draw.line()
        .points(vec2(0.0, model.hw * 0.125), vec2(0.0, -model.hw * 0.125) )
        .color(WHITE);

    draw.line()
        .points(vec2(0.0, model.change * 0.125), vec2(0.0, -model.change * 0.125) )
        .color(WHITE)
        .roll(TAU * 0.25);
         */

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,0.02));
    
    draw.to_frame(app, &frame).unwrap();
}