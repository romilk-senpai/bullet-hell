use crate::audio::AudioContext;
use crate::graphics::WgpuContext;
use crate::input::InputContext;
use crate::physics::PhysicsContext;

pub struct Engine {
    graphics: WgpuContext,
    physics: PhysicsContext,
    input: InputContext,
    audio: AudioContext,
}

impl Engine {
    pub fn new() -> Self {
        let graphics = WgpuContext {};
        let physics = PhysicsContext {};
        let input = InputContext {};
        let audio = AudioContext {};

        Self {
            graphics,
            physics,
            input,
            audio,
        }
    }

    pub fn run() {}
}
