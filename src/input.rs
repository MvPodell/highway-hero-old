use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Input {
    pub right_arrow_pressed: bool,
    pub left_arrow_pressed: bool,
    pub up_arrow_pressed: bool,
    pub down_arrow_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            right_arrow_pressed: false,
            left_arrow_pressed: false,
            up_arrow_pressed: false,
            down_arrow_pressed: false,
        }
    }

    pub fn handle_input_event(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput { input, .. } = event {
            if let Some(keycode) = input.virtual_keycode {
                match keycode {
                    VirtualKeyCode::Right => {
                        self.right_arrow_pressed = input.state == ElementState::Pressed;
                    }
                    VirtualKeyCode::Left => {
                        self.left_arrow_pressed = input.state == ElementState::Pressed;
                    }
                    VirtualKeyCode::Up => {
                        self.up_arrow_pressed = input.state == ElementState::Pressed;
                    }
                    VirtualKeyCode::Down => {
                        self.down_arrow_pressed = input.state == ElementState::Pressed;
                    }
                    _ => {}
                }
            }
        }
    }
}
