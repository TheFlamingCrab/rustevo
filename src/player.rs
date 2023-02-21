use crate::engine::input::{InputState};
use crate::engine::entity::Entity;
use crate::engine::graphics::{Vertex, FrameDescriptor};

pub struct Player {
    pub position: [f32; 2],
    pub rotation: f32,
}

impl Player {
    // REMOVE THIS FUNCTION LATER
    pub fn new() -> Self {
        Self {
            position: [0.0; 2],
            rotation: 0.0,
        }
    }
}

impl Entity for Player {
    fn start(&mut self) {
        todo!()
    }

    fn update(&mut self, descriptor: &FrameDescriptor, input: &InputState) {
        //self.rotation += (10.00 * descriptor.delta_time) as f32;
        let mouse_coords = input.get_mouse_coords();
        let x_clamped = (mouse_coords[0] - (descriptor.size.width as f64 / 2.0)) / (descriptor.size.width as f64 / 2.0);
        let y_clamped = (mouse_coords[1] - (descriptor.size.height as f64 / 2.0)) / (descriptor.size.height as f64 / 2.0);
        let x_diff = (x_clamped) - self.position[0] as f64;
        let y_diff = (y_clamped) - self.position[1] as f64;

        let mut angle: f64 = libm::atan(y_diff / x_diff);

        if angle < 0.0 {
            angle += std::f64::consts::PI;
        }

        if y_clamped > self.position[1] as f64 {
            angle += std::f64::consts::PI;
        }
 
        println!("{}", angle);

        self.rotation = -angle as f32;

        let x_speed = 1.0 * descriptor.delta_time;
        let y_speed = 1.0 * descriptor.delta_time;

        let x_input: i8 = input.get_key_held(2) as i8 - input.get_key_held(0) as i8;
        let y_input: i8 = input.get_key_held(13) as i8 - input.get_key_held(1) as i8;

        let x_change = ((x_input as f64 * x_speed)) as f32;
        let y_change = ((y_input as f64 * y_speed)) as f32;

        let magnitude = (x_change * x_change + y_change * y_change).sqrt();

        if magnitude != 0.0 {
            self.position[0] += (20.0 / descriptor.size.width as f32) * (x_change / magnitude);
            self.position[1] += (20.0 / descriptor.size.height as f32) * (y_change / magnitude);
        }
    }

    fn get_vertices(&self) -> Vec<Vertex> {
        let vertices: Vec<Vertex> = vec!(
            // Changed
            Vertex { position: [self.position[0] - 0.5, self.position[1] + 0.5, 0.0], tex_coords: [0.0, 0.0], pivot: [self.position[0], self.position[1]], rotation: self.rotation}, // A
            Vertex { position: [self.position[0] + 0.5, self.position[1] + 0.5, 0.0], tex_coords: [1.0, 0.0], pivot: [self.position[0], self.position[1]], rotation: self.rotation}, // B
            Vertex { position: [self.position[0] + 0.5, self.position[1] - 0.5, 0.0], tex_coords: [1.0, 1.0], pivot: [self.position[0], self.position[1]], rotation: self.rotation}, // C
            Vertex { position: [self.position[0] - 0.5, self.position[1] - 0.5, 0.0], tex_coords: [0.0, 1.0], pivot: [self.position[0], self.position[1]], rotation: self.rotation}, // D
        );

        vertices
    }
}