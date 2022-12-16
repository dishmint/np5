use nannou::prelude::*;

/* MODS */
mod emitter;
use crate::emitter::Emitter;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}

struct Model {
    emitter: Emitter /* TODO: array of emitters */
}

fn model(app: &App) -> Model {
    let emitter = Emitter::new(app);
    Model { emitter }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.emitter.update(app);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(srgb(0.698,0.133,0.133));
    }
    
    model.emitter.view(&draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.698,0.133,0.133,0.05));
    
    draw.to_frame(app, &frame).unwrap();
}