use core::universe::types::{Configuration, Coordinates, Universe};
use lazy_static::lazy_static;
use nannou::{draw::mesh::vertex::Color, prelude::*};
use nannou_egui::{self, egui, Egui};
use std::sync::Mutex;

lazy_static! {
    static ref STATE_FILE: Mutex<String> = Mutex::new(String::new());
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

pub enum State {
    Running,
    Paused,
}

pub struct Model {
    pub state: State,
    pub egui: Egui,
    pub win_w: f32,
    pub win_h: f32,
    pub block_size: f32,
    pub block_stroke: f32,
    pub cols: i32,
    pub rows: i32,
    pub show_numbers: bool,
    pub universe_file: String,
    pub universe_measure_max: usize,
    pub universe: Universe,
    pub selected_configuration: Option<usize>,
}

pub fn run(state_file: String) {
    *STATE_FILE.lock().unwrap() = state_file;
    nannou::app(model).update(update).view(view).run();
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.egui.draw_to_frame(&frame).unwrap();
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.egui.begin_frame();

    egui::Window::new("controls")
        .resizable(false)
        .show(&ctx, |ui| {
            ui.horizontal(|ui| match model.state {
                State::Running => {
                    if ui.button("Pause").clicked() {
                        model.state = State::Paused;
                    }
                }
                State::Paused => {
                    if ui.button("Reset").clicked() {
                        model.selected_configuration = None;
                        model.universe =
                            Universe::new_from_files(model.universe_file.as_str()).unwrap();
                        model.universe.step();
                    }
                    if ui.button("Run").clicked() {
                        model.state = State::Running;
                    }
                    if ui.button("Step").clicked() {
                        model.universe.step();
                    }
                    if ui.button("Measure").clicked() {
                        model.universe.measure();
                        model.selected_configuration = None;
                    }
                }
            });
            ui.separator();
            ui.label(format!("Step: {}", model.universe.step_count));
            ui.label(format!("Is even step: {}", model.universe.is_even_step));
            ui.separator();
            ui.label(format!(
                "Configurations count: {}",
                model.universe.state.len()
            ));
            ui.add_space(4.0);
            ui.checkbox(&mut model.show_numbers, "Show numbers");
            let row_height = 10.;
            let num_rows = model.universe.state.len();
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    ui.selectable_value(&mut model.selected_configuration, None, "Combined state");
                    for row in row_range {
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut model.selected_configuration,
                                Some(row),
                                format!(
                                    "Configuration: {}, amplitude: {:.4}",
                                    row + 1,
                                    model.universe.state[row].amplitude
                                ),
                            );
                        });
                        if row >= model.universe_measure_max {
                            ui.label(format!(
                                "can't show more than {} items",
                                model.universe_measure_max
                            ));
                            break;
                        }
                    }
                });
        });
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title("nannou web test")
        .size(WIDTH, HEIGHT)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let egui_window_ref = app.window(main_window).unwrap();
    let egui = Egui::from_window(&egui_window_ref);

    //Create a universe from a file
    let state_file = STATE_FILE.lock().unwrap();
    let universe: Universe = Universe::new_from_files(&state_file).unwrap();

    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();
    let block_size = 30.;
    let block_stroke = 0.2;
    let cols = (win_w / block_size).ceil() as i32;
    let rows = (win_h / block_size).ceil() as i32;

    Model {
        state: State::Running,
        egui,
        win_w,
        win_h,
        block_size,
        block_stroke,
        cols,
        rows,
        show_numbers: false,
        universe_file: state_file.to_string(),
        universe_measure_max: 128,
        selected_configuration: None,
        universe,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    match model.state {
        State::Running => {
            let frame_to_skip = 10;
            // Since we are unable to set the frame rate of the nannou app
            // we use this trick to skip some frames in order to slow down
            // the simulation. You can find out more in this comment: https://github.com/nannou-org/nannou/issues/708#issuecomment-1047032678
            if app.elapsed_frames() % frame_to_skip != 0 {
                return;
            } else {
                if model.universe.state.len() > model.universe_measure_max {
                    model.universe.measure();
                }

                model.universe.step();
            }
        }
        State::Paused => {
            if model.universe.state.len() > model.universe_measure_max {
                model.universe.measure();
            }
        }
    }

    update_ui(model);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let universe = &model.universe;
    let draw = app.draw();
    let m = &model;

    let gray = Color::new(22. / 255., 27. / 255., 34. / 255., 1.);
    draw.background().color(gray);

    let gdraw = draw.scale_y(1.0).x_y(
        m.block_size / 2. - m.win_w / 2.0,
        m.block_size / 2. - m.win_h / 2.0,
    );

    for i in 0..m.cols {
        for j in 0..m.rows {
            match m.selected_configuration {
                None => {
                    draw_combined_state(i, j, universe, &gdraw, m);
                }
                Some(index) => {
                    if index >= universe.state.len() {
                        continue;
                    }
                    draw_configuration(i, j, &universe.state[index], &gdraw, m);
                }
            }
        }
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
    ui_view(app, model, frame);
}

fn draw_combined_state(i: i32, j: i32, universe: &Universe, gdraw: &Draw, m: &Model) {
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

            if m.show_numbers {
                gdraw
                    .text(format!("{:.2}", *probability as f32).as_str())
                    .x_y(i as f32 * (m.block_size), j as f32 * (m.block_size))
                    .font_size(12)
                    .color(BLACK);
            }
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

fn draw_configuration(i: i32, j: i32, configuration: &Configuration, gdraw: &Draw, m: &Model) {
    match configuration.living_cells.get(&Coordinates { x: i, y: j }) {
        Some(_) => {
            //draw living cells
            let green = Color::new(0.0, 1., 0.0, 1.);
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
