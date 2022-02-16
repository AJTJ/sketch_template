use nannou::prelude::*;
use rand::prelude::*;
use std::path::Path;

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

    draw.background().color(BLUE);

    // DRAW TO FRAME
    draw.to_frame(app, &frame).unwrap();

    // CAPTURE FRAME
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    // capture all the frames to a directory outside of the current crate
    Path::new("../sketch_frames")
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
