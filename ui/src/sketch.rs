use core::universe::types::{Coordinates, Universe};
use nannou::draw::mesh::vertex::Color;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model {
    pub is_web: bool,
    pub universe: Universe,
}

const STROKE_WEIGHT: f32 = 0.2;

fn update(app: &App, model: &mut Model, _update: Update) {
    let frame_to_skip = if model.is_web { 5 } else { 10 };
    // Since we are unable to set the frame rate of the nannou app
    // we use this trick to skip some frames in order to slow down
    // the simulation. You can find out more in this comment: https://github.com/nannou-org/nannou/issues/708#issuecomment-1047032678
    if app.elapsed_frames() % frame_to_skip != 0 {
        return;
    }

    if model.universe.state.len() > 128 {
        model.universe.measure();
    }
    model.universe.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window_width = app.window_rect().w();
    let window_height = app.window_rect().h();
    let s_x = 30.;
    let s_y = 30.;
    let cols = (window_width / s_x).ceil() as i32;
    let rows = (window_height / s_y).ceil() as i32;
    let universe = &model.universe;
    let draw = app.draw();

    let gray = Color::new(22. / 255., 27. / 255., 34. / 255., 1.);
    draw.background().color(gray);
    let gdraw = draw.scale_y(-1.0).x_y(
        s_x / 2. - window_width / 2.0,
        s_y / 2. - window_height / 2.0,
    );

    for i in 0..cols {
        for j in 0..rows {
            match universe.combined_state.get(&Coordinates { x: i, y: j }) {
                Some(probability) => {
                    //draw living cells
                    let green = Color::new(0.0, 1., 0.0, *probability as f32);
                    gdraw
                        .rect()
                        .stroke(GRAY)
                        .stroke_weight(STROKE_WEIGHT)
                        .x_y(i as f32 * (s_x), j as f32 * (s_y))
                        .w_h(s_x, s_y)
                        .color(green);
                }
                None => {
                    gdraw
                        .rect()
                        .no_fill()
                        .stroke(GRAY)
                        .stroke_weight(STROKE_WEIGHT)
                        .x_y(i as f32 * (s_x), j as f32 * (s_y))
                        .w_h(s_x, s_y);
                }
            }
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app(is_web: bool, width: u32, height: u32) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    //Create a universe from a file
    let universe: Universe = if is_web {
        Universe::new_from_str(include_str!(
            "../../core/fixtures/state_2_diagonal_cells.json"
        ))
        .unwrap()
    } else {
        let state_file = "./core/fixtures/state_2_diagonal_cells.json";
        Universe::new_from_files(state_file).unwrap()
    };

    let model = Model { is_web, universe };
    MODEL.with(|m| m.borrow_mut().replace(model));

    app::Builder::new_async(move |app| {
        Box::new(async move {
            create_window(app, width, height).await;
            MODEL.with(|m| m.borrow_mut().take().unwrap())
        })
    })
    .backends(Backends::PRIMARY | Backends::GL)
    .update(update)
    .run_async()
    .await;
}

async fn create_window(app: &App, width: u32, height: u32) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.set_loop_mode(LoopMode::rate_fps(5.0));
    app.new_window()
        .device_descriptor(device_desc)
        .title("nannou web test")
        .size(width, height)
        // .raw_event(raw_event)
        // .key_pressed(key_pressed)
        // .key_released(key_released)
        // .mouse_pressed(mouse_pressed)
        // .mouse_moved(mouse_moved)
        // .mouse_released(mouse_released)
        // .mouse_wheel(mouse_wheel)
        // .touch(touch)
        .view(view)
        .build_async()
        .await
        .unwrap();
}
