use zxcengine::context::run;

fn main() {
    pollster::block_on(run());
}
