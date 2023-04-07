use nannou::{prelude::*, draw::mesh::vertex::Color};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    let col = 100;
    let row = 100;
    let s_x = 30.;
    let s_y = 30.;

//     let gray = Color::new(22. / 255., 27. / 255., 34. / 255. ,255. / 255.);
//     draw.background().color(gray);
    draw.background().color(WHITE);
    let gdraw = draw.scale_y(-1.0).x_y(
        s_x / 2. - (WIDTH as f32) / 2.0,
        s_y / 2. - (HEIGHT as f32) / 2.0,
    );

    for i in 0..col {
        for j in 0..row {
            let r = random_range(0, 10);
            if r >= 5 {
                gdraw
//                     .ellipse()
                  .rect()
                    .no_fill()
//                     .stroke(BLACK)
//                     .stroke_weight(2.)
                    .x_y(i as f32 * (s_x), j as f32 * (s_y))
                    .w_h(s_x, s_y);
            } else {
                let green = Color::new(0.0, random_range(0.7, 0.1), 0.0, random_range(0., 1.));
                gdraw
                    .rect()
//                     .ellipse()
//                     .stroke(BLACK)
//                     .stroke_weight(2.)
                    .x_y(i as f32 * (s_x), j as f32 * (s_y))
                    .w_h(s_x, s_y)
                    .color(green);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
