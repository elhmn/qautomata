use core::universe::types::{Coordinates, Universe};
use nannou::draw::mesh::vertex::Color;
use nannou::prelude::*;
use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model {
    pub win_w: f32,
    pub win_h: f32,
    pub block_size: f32,
    pub block_stroke: f32,
    pub cols: i32,
    pub rows: i32,
    pub is_webapp: bool,
    pub universe: Universe,
}

impl Model {
    pub fn new(app: &App, is_webapp: bool) -> Self {
        //Create a universe from a file
        let universe: Universe = if is_webapp {
            Universe::new_from_str(include_str!(
                "../../core/fixtures/state_2_diagonal_cells.json"
            ))
            .unwrap()
        } else {
            let state_file = "./core/fixtures/state_2_diagonal_cells.json";
            Universe::new_from_files(state_file).unwrap()
        };

        let win_w = app.window_rect().w();
        let win_h = app.window_rect().h();
        let block_size = 30.;
        let block_stroke = 0.2;
        let cols = (win_w / block_size).ceil() as i32;
        let rows = (win_h / block_size).ceil() as i32;

        Self {
            win_w,
            win_h,
            block_size,
            block_stroke,
            cols,
            rows,
            is_webapp,
            universe,
        }
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let frame_to_skip = 10;
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
    let universe = &model.universe;
    let draw = app.draw();
    let m = &model;

    let gray = Color::new(22. / 255., 27. / 255., 34. / 255., 1.);
    draw.background().color(gray);

    let gdraw = draw.scale_y(-1.0).x_y(
        m.block_size / 2. - m.win_w / 2.0,
        m.block_size / 2. - m.win_h / 2.0,
    );

    for i in 0..m.cols {
        for j in 0..m.rows {
            match universe.combined_state.get(&Coordinates { x: i, y: j }) {
                Some(probability) => {
                    //draw living cells
                    let green = Color::new(0.0, 1., 0.0, *probability as f32);
                    gdraw
                        .rect()
                        .stroke(GRAY)
                        .stroke_weight(m.block_stroke)
                        .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                        .w_h(m.block_size, m.block_size)
                        .color(green);
                }
                None => {
                    gdraw
                        .rect()
                        .no_fill()
                        .stroke(GRAY)
                        .stroke_weight(m.block_stroke)
                        .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                        .w_h(m.block_size, m.block_size);
                }
            }
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app(is_webapp: bool, width: u32, height: u32) {
    // Since ModelFn is not a closure we need this workaround to pass the calculated model
    thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

    app::Builder::new_async(move |app| {
        Box::new(async move {
            create_window(app, width, height).await;
            let model = Model::new(app, is_webapp);
            MODEL.with(|m| m.borrow_mut().replace(model));
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
        .view(view)
        .build_async()
        .await
        .unwrap();
}
