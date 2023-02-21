use winit::{event::ElementState, dpi::PhysicalPosition};

pub struct InputState {
    keys_pressed: [bool; 128],
    mouse_coords: [f64; 2],
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_pressed: [false; 128],
            mouse_coords: [0.0; 2]
        }
    }

    pub fn set_mouse_coords(&mut self, position: [f64; 2]) {
        self.mouse_coords = position;
    }

    pub fn get_mouse_coords(&self) -> [f64; 2] {
        self.mouse_coords
    }

    pub fn set_key_held(&mut self, keycode: usize, state: bool) {
        self.keys_pressed[keycode] = state;
    }

    pub fn get_key_held(&self, keycode: usize) -> bool {
        self.keys_pressed[keycode]
    }
}

pub trait InputHandler {
    fn handle_input(state: ElementState, keycode: u32);
}