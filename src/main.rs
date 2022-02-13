use nannou::prelude::*;
use rand::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(1, 1)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(app: &App, _model: &mut Model, _update: Update) {
    // move it to the corner of the screen
    app.main_window().set_outer_position_pixels(0, 0);

    // resize it so that it is now visible
    app.main_window().set_inner_size_pixels(1000, 1000);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // GENERAL
    let draw = app.draw();

    // DRAW TO FRAME
    draw.to_frame(app, &frame).unwrap();
}
