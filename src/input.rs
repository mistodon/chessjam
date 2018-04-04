pub use glium::glutin::VirtualKeyCode as Key;
pub use glium::glutin::ModifiersState;


#[derive(Clone)]
pub struct Keyboard {
    pub modifiers: ModifiersState,
    keys_down: [bool; 256],
    keys_pressed: [bool; 256],
    keys_released: [bool; 256]
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard
        {
            modifiers: ModifiersState::default(),
            keys_down: [false; 256],
            keys_pressed: [false; 256],
            keys_released: [false; 256]
        }
    }
}

impl Keyboard
{
    pub fn begin_frame_input(&mut self) -> KeyboardInput
    {
        self.keys_pressed = [false; 256];
        self.keys_released = [false; 256];
        KeyboardInput { keyboard: self }
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


pub struct KeyboardInput<'a> {
    keyboard: &'a mut Keyboard
}

impl<'a> KeyboardInput<'a> {
    pub fn press(&mut self, key: Key, modifiers: ModifiersState)
    {
        self.keyboard.keys_down[key as usize] = true;
        self.keyboard.keys_pressed[key as usize] = true;
        self.keyboard.modifiers = modifiers;
    }

    pub fn release(&mut self, key: Key, modifiers: ModifiersState)
    {
        self.keyboard.keys_down[key as usize] = false;
        self.keyboard.keys_released[key as usize] = true;
        self.keyboard.modifiers = modifiers;
    }
}
