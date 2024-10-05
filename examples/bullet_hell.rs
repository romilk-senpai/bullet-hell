use zxcengine::engine::run;

fn main() {
    pollster::block_on(run());
}
