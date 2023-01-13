use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(500,500)
        .run();
}
struct Model {
    num_circles: i32,
    multiplier: f32,
    b_radius: f32,
    radius: f32,
    max_w: f32,
    b_color: Rgb<u8>,
    c_color: Srgb,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    let num_circles = 0;
    let multiplier = 10.0;
    let b_radius = multiplier * 0.5;
    let radius = b_radius;
    let max_w = win.w() * 0.75;

    /* BASE COLOR */
    let b_color = WHITE;
    /* CIRCLE COLOR */
    let c_color = srgb(60.0/255.0, 139.0/255.0, 108.0/255.0);

    Model {num_circles, multiplier, b_radius, radius, max_w, b_color, c_color }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 1..=model.num_circles {
        model.radius = (i as f32 * model.multiplier) * 0.5;
    }

    model.num_circles += 1;

    if model.multiplier * model.num_circles as f32 > model.max_w {
        model.num_circles = 0;
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    if app.elapsed_frames() < 1 {
        draw.background().color(BLACK);
    }
    
    if model.num_circles == 0 {
        draw.ellipse()
            /* .no_fill() */
            .color(model.b_color)
            .stroke_color(model.b_color)
            .stroke_weight(1.0)
            .radius(model.b_radius);
    } else {
        for _ in 1..=model.num_circles {
            draw.ellipse()
                .no_fill()
                .stroke_color(model.c_color)
                .stroke_weight(1.0)
                .radius(model.radius);
        }
    }

    draw.rect()
        .wh(win.wh())
        .color(srgba(0.0,0.0,0.0,model.multiplier/255.0));
    draw.to_frame(app, &frame).unwrap();
}