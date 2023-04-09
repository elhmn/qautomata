// native app entry_point

use async_std::task::block_on;

use sketch::run_app;

mod sketch;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn main() {
    block_on(async {
        run_app(false, WIDTH, HEIGHT).await;
    });
}
