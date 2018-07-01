use std::collections::HashMap;

use uni_app::AppEvent;

/// Provides information about user input.
/// Possible values for the `key` scancode parameter can be found in unrust/uni-app's `translate_scan_code`
/// [function](https://github.com/unrust/uni-app/blob/41246b070567e3267f128fff41ededf708149d60/src/native_keycode.rs#L160).
/// Warning, there are some slight variations from one OS to another, for example the `Command`, `F13`, `F14`, `F15` keys
/// only exist on Mac.
///
pub trait InputApi {
    // keyboard
    /// return the current status of a key (true if pressed)
    fn key(&self, key: &str) -> bool;
    /// return true if a key was pressed since last update.
    /// Call this function only once per update. Once it is called, the status is cleared and it will return false.
    fn key_pressed(&mut self, key: &str) -> bool;
    /// return true if a key was released since last update.
    /// Call this function only once per update. Once it is called, the status is cleared and it will return false.
    fn key_released(&mut self, key: &str) -> bool;
    // mouse
    /// return the current status of a mouse button (true if pressed)
    fn mouse_button(&self, num: usize) -> bool;
    /// return true if a mouse button was pressed since last update.
    /// Call this function only once per update. Once it is called, the status is cleared and it will return false.
    fn mouse_button_pressed(&mut self, num: usize) -> bool;
    /// return true if a mouse button was released since last update.
    /// Call this function only once per update. Once it is called, the status is cleared and it will return false.
    fn mouse_button_released(&mut self, num: usize) -> bool;
    /// return the current mouse position in pixels and in console cells coordinates from the game window top left corner
    /// (pix_x, pix_y, con_x, con_y)
    fn mouse_pos(&self) -> (f32, f32, f32, f32);
}

pub struct DoryenInput {
    kdown: HashMap<String, bool>,
    kpressed: HashMap<String, bool>,
    kreleased: HashMap<String, bool>,
    mdown: HashMap<usize, bool>,
    mpressed: HashMap<usize, bool>,
    mreleased: HashMap<usize, bool>,
    mpos: (f32, f32, f32, f32),
    screen_size: (f32, f32),
    char_size: (u32, u32),
}

impl DoryenInput {
    pub fn new(screen_width: u32, screen_height: u32) -> DoryenInput {
        DoryenInput {
            kdown: HashMap::new(),
            kpressed: HashMap::new(),
            kreleased: HashMap::new(),
            mdown: HashMap::new(),
            mpressed: HashMap::new(),
            mreleased: HashMap::new(),
            mpos: (0.0, 0.0, 0.0, 0.0),
            screen_size: (screen_width as f32, screen_height as f32),
            char_size: (1, 1),
        }
    }
    fn on_key_down(&mut self, code: &str) {
        if !self.key(code) {
            self.kpressed.insert(code.to_owned(), true);
            self.kdown.insert(code.to_owned(), true);
        }
    }
    fn on_key_up(&mut self, code: &String) {
        self.kpressed.insert(code.clone(), false);
        self.kdown.insert(code.clone(), false);
        self.kreleased.insert(code.clone(), true);
    }
    fn on_mouse_down(&mut self, button: usize) {
        if !self.mouse_button(button) {
            self.mpressed.insert(button, true);
            self.mdown.insert(button, true);
        }
    }
    fn on_mouse_up(&mut self, button: usize) {
        self.mpressed.insert(button, false);
        self.mdown.insert(button, false);
        self.mreleased.insert(button, true);
    }
    pub fn on_frame(&mut self) {
        self.mpressed.clear();
        self.mreleased.clear();
    }
    pub fn on_event(&mut self, event: &AppEvent) {
        match event {
            &AppEvent::KeyDown(ref key) => {
                self.on_key_down(&key.code);
            }
            &AppEvent::KeyUp(ref key) => {
                self.on_key_up(&key.code);
            }
            &AppEvent::MousePos(ref pos) => {
                self.mpos = (
                    pos.0 as f32 / self.screen_size.0,
                    pos.1 as f32 / self.screen_size.1,
                    pos.0 as f32 / self.char_size.0 as f32,
                    pos.1 as f32 / self.char_size.1 as f32,
                );
            }
            &AppEvent::MouseDown(ref mouse) => {
                self.on_mouse_down(mouse.button);
            }
            &AppEvent::MouseUp(ref mouse) => {
                self.on_mouse_up(mouse.button);
            }
            &AppEvent::Resized(size) => {
                self.resize(size);
            }
        }
    }
    fn resize(&mut self, size: (u32, u32)) {
        self.screen_size = (size.0 as f32, size.1 as f32);
    }
    pub fn set_char_size(&mut self, width: u32, height: u32) {
        self.char_size = (width, height);
    }
}

impl InputApi for DoryenInput {
    fn key(&self, key: &str) -> bool {
        match self.kdown.get(key) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn key_pressed(&mut self, key: &str) -> bool {
        match self.kpressed.get(key) {
            Some(&true) => {
                self.kpressed.insert(key.to_owned(), false);
                return true;
            }
            _ => false,
        }
    }
    fn key_released(&mut self, key: &str) -> bool {
        match self.kreleased.get(key) {
            Some(&true) => {
                self.kreleased.insert(key.to_owned(), false);
                return true;
            }
            _ => false,
        }
    }
    fn mouse_button(&self, num: usize) -> bool {
        match self.mdown.get(&num) {
            Some(&true) => true,
            _ => false,
        }
    }
    fn mouse_button_pressed(&mut self, num: usize) -> bool {
        match self.mpressed.get(&num) {
            Some(&true) => {
                self.mpressed.insert(num, false);
                return true;
            }
            _ => false,
        }
    }
    fn mouse_button_released(&mut self, num: usize) -> bool {
        match self.mreleased.get(&num) {
            Some(&true) => {
                self.mreleased.insert(num, false);
                return true;
            }
            _ => false,
        }
    }
    fn mouse_pos(&self) -> (f32, f32, f32, f32) {
        self.mpos
    }
}
