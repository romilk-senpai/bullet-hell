use zxcengine::{runner::*, Engine};

struct SampleGame {}

impl Handler for SampleGame {
    fn update(&self) {
        println!("update");
    }

    fn draw(&self) {
        println!("draw");
    }
}

fn main() {
    let ctx = Engine::new();
    let game = SampleGame {};
    let mut runner = Runner::new(game);

    runner.run(&ctx);
}
