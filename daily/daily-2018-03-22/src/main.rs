use nannou::prelude::*;

mod circleset;
use crate::circleset::CircleSet;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    escalate: f32, 
    dir: f32, 
    c1: CircleSet,
    c2: CircleSet,
    c3: CircleSet,
}

fn model(_app: &App) -> Model {
    let escelate = 0.0;
    let dir = 0.1;

    let clr1 = vec3(255.0, 255.0, 255.0);
    let clr3 = vec3(255.0, 99.0, 71.0);

    let clr2 = vec3(0.0,0.0,0.0).lerp(clr3, 0.5);

    let c1 = CircleSet::new(20, rgb(clr1.x as u8, clr1.y as u8, clr1.z as u8));
    let c2 = CircleSet::new(20, rgb(clr2.x as u8, clr2.y as u8, clr2.z as u8));
    let c3 = CircleSet::new(20, rgb(clr3.x as u8, clr3.y as u8, clr3.z as u8));
    Model {escalate: escelate, dir, c1, c2, c3}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    // model.c1.update(); /* don't need to update c1 */
    model.c2.update(model.escalate, 1.0);
    model.c3.update(model.escalate, 0.5);

    model.escalate += model.dir as f32;

    if model.escalate > win.right() || model.escalate < win.left() {
        model.dir *= -1.0;
      }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    model.c1.view(&draw);
    model.c2.view(&draw);
    model.c3.view(&draw);

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,clamp(1.0/abs(model.escalate), 0.01, 0.15)));


    draw.to_frame(app, &frame).unwrap();
}