use nannou::prelude::*;

mod liner;
use crate::liner::Liner;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    alpha: f32,
    liner: Liner
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let alpha = 0.1;
    let liner = Liner::new(win, 1.0);
    Model {alpha, liner}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.liner.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    let ow = win.w();

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }

    let qw = ow * 0.25;

    // Centered White Square
    draw.rect()
        .wh(vec2(qw, qw))
        .color(WHITE);

    // Centered White Horizontal Line
    draw.line()
        .start(vec2(- qw,0.0))
        .end(vec2(qw,0.0))
        .color(WHITE);

    // Centered Rotating Tomato Line
    model.liner.view(&draw);

    
    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,model.alpha));

    draw.to_frame(app, &frame).unwrap();
}