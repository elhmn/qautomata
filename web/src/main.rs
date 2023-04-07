use core::universe::types::{Coordinates, Universe};
use nannou::{draw::mesh::vertex::Color, prelude::*};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const STROKE_WEIGHT: f32 = 0.2;

fn main() {
    nannou::app(model).update(update).run();
}

#[warn(dead_code)]
struct Model {
    _win_id: WindowId,
    universe: Universe,
}

fn model(app: &App) -> Model {
    let win_id = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    //Create a universe from a file
    let state_file = "./core/fixtures/state_2_diagonal_cells.json";
    let universe = Universe::new_from_files(state_file).unwrap();

    Model {
        _win_id: win_id,
        universe,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.universe.state.len() > 128 {
        model.universe.measure();
    }
    model.universe.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let col = 100;
    let row = 100;
    let s_x = 30.;
    let s_y = 30.;
    let universe = &model.universe;
    let draw = app.draw();

    let gray = Color::new(22. / 255., 27. / 255., 34. / 255., 1.);
    draw.background().color(gray);
    //     draw.background().color(WHITE);
    let gdraw = draw.scale_y(-1.0).x_y(
        s_x / 2. - (WIDTH as f32) / 2.0,
        s_y / 2. - (HEIGHT as f32) / 2.0,
    );

    for i in 0..col {
        for j in 0..row {
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
                    //Draw dead cells
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

    draw.to_frame(app, &frame).unwrap();
}
