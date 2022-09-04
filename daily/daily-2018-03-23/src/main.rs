use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(400,400)
        .run();
}

struct Model {
    wh_max: Vec2,
    wh_outer: Vec2,
    wh_inner: Vec2,
    change: f32,
    dir: f32
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let wh_max = win.wh() * 0.99;
    let wh_outer = wh_max;
    let wh_inner = wh_max;
    let change = 0.0;
    let dir = 0.25;
    Model {wh_max, wh_outer, wh_inner, change, dir}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.change += model.dir;

    let tri = deg_to_rad(model.change)
            .sin()
            .asin() * (4.0/TAU);
    model.wh_inner = model.wh_max * tri;

    /* 
    
        This kind of update-function typically ends with an if statement which flips the direction a value changes when it exceeds a threshold.

        For example,
            ```
            // change = 0 ; direction = 1 ;

            change += direction;
            if change > 100 || change < 0 {
                direction *= -1;
            }
            ```
    
        I noticed I could use a trig function to avoid using if. sin() was helpful, but non-linear. Thus, a triangle function was used.
        There doesn't appear to be a tri function in nannou, so with the following bits of information I was able to implement it.
        
        /*  (2/PI) * asin(sin(x)) */ mathworld: https://mathworld.wolfram.com/TriangleWave.html
        /* (4/TAU) * asin(sin(x)) */       w|a: https://www.wolframalpha.com/input?i=%282%2FPI%29+%3D+%28x+%2F+%282+*+PI%29%29

        graphtoy for visual ref: https://graphtoy.com/?f1(x,t)=sin(x)&v1=true&f2(x,t)=(4/TAU)*asin(sin(x))&v2=true&f3(x,t)=&v3=false&f4(x,t)=&v4=false&f5(x,t)=&v5=false&f6(x,t)=&v6=false&grid=1&coords=1.5140383290428288,0.9823309111568392,5.598088562516882
    */

}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();

    // Draw Outer
    draw.rect()
        .no_fill()
        .stroke_weight(1.0)
        .stroke(RED)
        .wh(model.wh_outer);

    // Draw Inner
    draw.rect()
        .no_fill()
        .stroke_weight(1.0)
        .stroke(WHITE)
        .wh(model.wh_inner);


    draw.rect()
         .wh(win.wh())
         .color(srgba(0.0,0.0,0.0,0.08));

    draw.to_frame(app, &frame).unwrap();
}