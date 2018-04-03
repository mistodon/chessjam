pub use glium::glutin::VirtualKeyCode as Key;
pub use glium::glutin::ModifiersState;


#[derive(Clone)]
pub struct KeyboardState {
    pub modifiers: ModifiersState,
    keys_down: [bool; 256],
    keys_pressed: [bool; 256],
    keys_released: [bool; 256]
}

impl Default for KeyboardState {
    fn default() -> Self {
        KeyboardState
        {
            modifiers: ModifiersState::default(),
            keys_down: [false; 256],
            keys_pressed: [false; 256],
            keys_released: [false; 256]
        }
    }
}

impl KeyboardState
{
    pub fn begin_frame(&mut self)
    {
        self.keys_pressed = [false; 256];
        self.keys_released = [false; 256];
    }

    pub fn press(&mut self, key: Key, modifiers: ModifiersState)
    {
        self.keys_down[key as usize] = true;
        self.keys_pressed[key as usize] = true;
        self.modifiers = modifiers;
    }

    pub fn release(&mut self, key: Key, modifiers: ModifiersState)
    {
        self.keys_down[key as usize] = false;
        self.keys_released[key as usize] = true;
        self.modifiers = modifiers;
    }

    pub fn down(&self, key: Key) -> bool
    {
        self.keys_down[key as usize]
    }

    pub fn pressed(&self, key: Key) -> bool
    {
        self.keys_pressed[key as usize]
    }

    pub fn released(&self, key: Key) -> bool
    {
        self.keys_released[key as usize]
    }
}
