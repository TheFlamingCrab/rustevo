use crate::engine::graphics::Vertex;
use crate::engine::input::InputState;
use crate::engine::graphics::FrameDescriptor;

pub trait Entity {
    fn start(&mut self);

    fn update(&mut self, descriptor: &FrameDescriptor, input: &InputState);

    fn get_vertices(&self) -> Vec<Vertex>;
}