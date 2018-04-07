#![allow(dead_code)] // TODO(claire): Remove

pub use glium::glutin::ModifiersState;
pub use glium::glutin::MouseButton as Button;
pub use glium::glutin::VirtualKeyCode as Key;


#[derive(Clone)]
pub struct Keyboard {
    pub modifiers: ModifiersState,
    keys_down: [bool; 256],
    keys_pressed: [bool; 256],
    keys_released: [bool; 256],
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            modifiers: ModifiersState::default(),
            keys_down: [false; 256],
            keys_pressed: [false; 256],
            keys_released: [false; 256],
        }
    }
}

impl Keyboard {
    pub fn begin_frame_input(&mut self) -> KeyboardInput {
        self.keys_pressed = [false; 256];
        self.keys_released = [false; 256];
        KeyboardInput { keyboard: self }
    }

    pub fn down(&self, key: Key) -> bool {
        self.keys_down[key as usize]
    }

    pub fn pressed(&self, key: Key) -> bool {
        self.keys_pressed[key as usize]
    }

    pub fn released(&self, key: Key) -> bool {
        self.keys_released[key as usize]
    }
}


pub struct KeyboardInput<'a> {
    keyboard: &'a mut Keyboard,
}

impl<'a> KeyboardInput<'a> {
    pub fn press(&mut self, key: Key, modifiers: ModifiersState) {
        self.keyboard.keys_down[key as usize] = true;
        self.keyboard.keys_pressed[key as usize] = true;
        self.keyboard.modifiers = modifiers;
    }

    pub fn release(&mut self, key: Key, modifiers: ModifiersState) {
        self.keyboard.keys_down[key as usize] = false;
        self.keyboard.keys_released[key as usize] = true;
        self.keyboard.modifiers = modifiers;
    }
}


#[derive(Clone, Default)]
pub struct Mouse {
    position: [f64; 2],
    down_position: Option<[f64; 2]>,
    buttons_down: [bool; 8],
    buttons_pressed: [bool; 8],
    buttons_released: [bool; 8],
}

impl Mouse {
    pub fn begin_frame_input(&mut self) -> MouseInput {
        self.buttons_pressed = [false; 8];
        self.buttons_released = [false; 8];
        MouseInput { mouse: self }
    }

    pub fn position(&self) -> [f64; 2] {
        self.position
    }

    pub fn down_position(&self) -> Option<[f64; 2]> {
        self.down_position
    }

    pub fn down(&self, button: Button) -> bool {
        self.buttons_down[mouse_button_to_index(button)]
    }

    pub fn pressed(&self, button: Button) -> bool {
        self.buttons_pressed[mouse_button_to_index(button)]
    }

    pub fn released(&self, button: Button) -> bool {
        self.buttons_released[mouse_button_to_index(button)]
    }
}


fn mouse_button_to_index(button: Button) -> usize {
    match button {
        Button::Left => 0,
        Button::Middle => 1,
        Button::Right => 2,
        Button::Other(n) => n as usize,
    }
}


pub struct MouseInput<'a> {
    mouse: &'a mut Mouse,
}

impl<'a> MouseInput<'a> {
    pub fn move_cursor_to(&mut self, x: f64, y: f64) {
        self.mouse.position = [x, y];
    }

    pub fn press(&mut self, button: Button) {
        self.mouse.buttons_down[mouse_button_to_index(button)] = true;
        self.mouse.buttons_pressed[mouse_button_to_index(button)] = true;
        self.mouse.down_position = Some(self.mouse.position);
    }

    pub fn release(&mut self, button: Button) {
        self.mouse.buttons_down[mouse_button_to_index(button)] = false;
        self.mouse.buttons_released[mouse_button_to_index(button)] = true;
        self.mouse.down_position = None;
    }
}
