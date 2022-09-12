use nannou::prelude::*;

mod square;
use crate::square::Square;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    square1: Square,
    square2: Square,
    square3: Square,
    alpha: f32,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let s = win.w() * 0.125;
    let radius = (((2.0 * pow(s,2)).sqrt()) - (s/2.0)) * 0.50;

    let sdf = 0.60;

    let obsp =    s * 0.01;
    let hbsp = obsp *  sdf;
    let qbsp = hbsp *  sdf;

    let black = srgb(020,020,023); /* Eerie Black */
    let green = srgb(060,139,108); /* Illuminating Emerald */
    let red   = srgb(216,082,082); /* Indian Red */

    let square1 = Square::new(s, obsp, radius, black);
    let square2 = Square::new(s * 0.66, hbsp, radius * 3.75, green);
    let square3 = Square::new(s * 0.33, qbsp, radius * 6.00, red);
    
    let alpha = obsp * 0.01;
    Model {square1, square2, square3, alpha}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.time;
    model.square1.update(time);
    model.square2.update(time);
    model.square3.update(time);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }

    model.square1.view(&draw);
    model.square2.view(&draw);
    model.square3.view(&draw);

    draw.rect()
        .wh(win.wh())
        // .color(srgba(1.0,1.0,1.0,model.alpha));
        .color(srgba(0.0,0.0,0.0,model.alpha));

    draw.to_frame(app, &frame).unwrap();
}