use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    wx: f64,
    speed: f64,
    delta: f64,
    dir: f64,
    x: f32, 
    y: f32
}

fn model(app: &App) -> Model {
    let wx = 100.00;
    let speed = 10.00;
    let delta = 0.0;
    let dir = 1.0;

    let center = app.window_rect().xy();

    let x = center.x;
    let y = center.y;

    Model {wx, speed, delta, dir, x, y}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    model.delta += 1.0;
    model.wx += model.speed * model.dir;


    model.x = model.wx as f32 * deg_to_rad(model.delta as f32 + ((random::<f32>() * 10.0 ) - 5.0)).cos();
    model.y = model.wx as f32 * deg_to_rad(model.delta as f32).sin();

    if model.wx > win.w() as f64 || model.wx < 20.0 {
		model.dir *= -1.0
	}
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    let wdt = win.w() / 2.0;

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.08,0.08,0.08,0.08));

    draw.ellipse()
        .radius(wdt * 0.5)
        .no_fill()
        .stroke_weight(0.5) /* NOTE: the stroke_weight was too small, that seems to be why I couldn't see it initially */
        .stroke_color(RED);

    draw.ellipse()
        .x_y(-1.0*model.x, model.y) /* NOTE: had to flip the x value to make this ellipse move clockwise. */
        .wh(vec2(model.wx as f32,model.wx as f32))
        .no_fill()
        .stroke_weight(0.5)
        .stroke_color(RED);

        let iwx = model.wx as i32;
        let iwd = wdt as i32;
    for i in (iwx..(iwd * 2)).step_by(model.wx as usize) {
        draw.ellipse()
        .x_y((i - iwd) as f32, 0.0)
        .wh(vec2(model.wx as f32,model.wx as f32))
        .color(BLACK)
        .stroke_weight(0.5)
        .stroke_color(RED);
    }

    draw.to_frame(app, &frame).unwrap();
}