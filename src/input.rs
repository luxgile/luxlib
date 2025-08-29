use std::ops::Index;

use log::{error, warn};
use winit::{
    event::{self, DeviceId, KeyEvent},
    keyboard::KeyCode,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KeyInput {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Up,
    Down,
    Left,
    Right,
    Escape,
    Space,
    Enter,
    Backspace,
    Delete,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Semicolon,
    Equal,
    Minus,
    Comma,
    Period,
    Slash,
    Backslash,
    Quote,
    BracketLeft,
    BracketRight,
    Backquote,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    Unknown, // For any keys we don't want to handle.
}

#[derive(Default, Debug)]
pub struct Input {
    keys_just_pressed: Vec<KeyInput>,
    keys_pressed: Vec<KeyInput>,
    keys_just_released: Vec<KeyInput>,
}
impl Input {
    pub fn advance(&mut self) {
        self.keys_pressed.extend_from_slice(&self.keys_just_pressed);
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
    }

    fn press_key(&mut self, key: KeyInput) {
        if self.keys_pressed.contains(&key) || self.keys_just_pressed.contains(&key) {
            return;
        }
        self.keys_just_pressed.push(key);
    }

    fn release_key(&mut self, key: KeyInput) {
        if !self.keys_pressed.contains(&key) {
            return;
        }
        self.keys_pressed.retain(|&x| x != key);
        self.keys_just_released.push(key);
    }

    pub fn is_key_just_pressed(&self, key: KeyInput) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    pub fn is_key_pressed(&self, key: KeyInput) -> bool {
        self.keys_just_pressed.contains(&key) || self.keys_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyInput) -> bool {
        self.keys_just_released.contains(&key)
    }

    pub fn is_key_released(&self, key: KeyInput) -> bool {
        !self.is_key_pressed(key)
    }

    pub(crate) fn handle_keyboard_input(
        &mut self,
        event: &KeyEvent,
        _device_id: &DeviceId,
        _is_synthetic: bool,
    ) {
        let input_key = match event.physical_key {
            winit::keyboard::PhysicalKey::Unidentified(native_key_code) => None,
            winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                KeyCode::KeyA => Some(KeyInput::A),
                KeyCode::KeyB => Some(KeyInput::B),
                KeyCode::KeyC => Some(KeyInput::C),
                KeyCode::KeyD => Some(KeyInput::D),
                KeyCode::KeyE => Some(KeyInput::E),
                KeyCode::KeyF => Some(KeyInput::F),
                KeyCode::KeyG => Some(KeyInput::G),
                KeyCode::KeyH => Some(KeyInput::H),
                KeyCode::KeyI => Some(KeyInput::I),
                KeyCode::KeyJ => Some(KeyInput::J),
                KeyCode::KeyK => Some(KeyInput::K),
                KeyCode::KeyL => Some(KeyInput::L),
                KeyCode::KeyM => Some(KeyInput::M),
                KeyCode::KeyN => Some(KeyInput::N),
                KeyCode::KeyO => Some(KeyInput::O),
                KeyCode::KeyP => Some(KeyInput::P),
                KeyCode::KeyQ => Some(KeyInput::Q),
                KeyCode::KeyR => Some(KeyInput::R),
                KeyCode::KeyS => Some(KeyInput::S),
                KeyCode::KeyT => Some(KeyInput::T),
                KeyCode::KeyU => Some(KeyInput::U),
                KeyCode::KeyV => Some(KeyInput::V),
                KeyCode::KeyW => Some(KeyInput::W),
                KeyCode::KeyX => Some(KeyInput::X),
                KeyCode::KeyY => Some(KeyInput::Y),
                KeyCode::KeyZ => Some(KeyInput::Z),
                KeyCode::Digit0 => Some(KeyInput::Digit0),
                KeyCode::Digit1 => Some(KeyInput::Digit1),
                KeyCode::Digit2 => Some(KeyInput::Digit2),
                KeyCode::Digit3 => Some(KeyInput::Digit3),
                KeyCode::Digit4 => Some(KeyInput::Digit4),
                KeyCode::Digit5 => Some(KeyInput::Digit5),
                KeyCode::Digit6 => Some(KeyInput::Digit6),
                KeyCode::Digit7 => Some(KeyInput::Digit7),
                KeyCode::Digit8 => Some(KeyInput::Digit8),
                KeyCode::Digit9 => Some(KeyInput::Digit9),
                KeyCode::ArrowUp => Some(KeyInput::Up),
                KeyCode::ArrowDown => Some(KeyInput::Down),
                KeyCode::ArrowLeft => Some(KeyInput::Left),
                KeyCode::ArrowRight => Some(KeyInput::Right),
                KeyCode::Space => Some(KeyInput::Space),
                KeyCode::Enter => Some(KeyInput::Enter),
                KeyCode::Escape => Some(KeyInput::Escape),
                KeyCode::Backspace => Some(KeyInput::Backspace),
                KeyCode::Delete => Some(KeyInput::Delete),
                KeyCode::Tab => Some(KeyInput::Tab),
                KeyCode::Home => Some(KeyInput::Home),
                KeyCode::End => Some(KeyInput::End),
                KeyCode::PageUp => Some(KeyInput::PageUp),
                KeyCode::PageDown => Some(KeyInput::PageDown),
                KeyCode::Insert => Some(KeyInput::Insert),
                KeyCode::Semicolon => Some(KeyInput::Semicolon),
                KeyCode::Equal => Some(KeyInput::Equal),
                KeyCode::Minus => Some(KeyInput::Minus),
                KeyCode::Comma => Some(KeyInput::Comma),
                KeyCode::Period => Some(KeyInput::Period),
                KeyCode::Slash => Some(KeyInput::Slash),
                KeyCode::Backslash => Some(KeyInput::Backslash),
                KeyCode::Quote => Some(KeyInput::Quote),
                KeyCode::BracketLeft => Some(KeyInput::BracketLeft),
                KeyCode::BracketRight => Some(KeyInput::BracketRight),
                KeyCode::Backquote => Some(KeyInput::Backquote),
                KeyCode::Numpad0 => Some(KeyInput::Numpad0),
                KeyCode::Numpad1 => Some(KeyInput::Numpad1),
                KeyCode::Numpad2 => Some(KeyInput::Numpad2),
                KeyCode::Numpad3 => Some(KeyInput::Numpad3),
                KeyCode::Numpad4 => Some(KeyInput::Numpad4),
                KeyCode::Numpad5 => Some(KeyInput::Numpad5),
                KeyCode::Numpad6 => Some(KeyInput::Numpad6),
                KeyCode::Numpad7 => Some(KeyInput::Numpad7),
                KeyCode::Numpad8 => Some(KeyInput::Numpad8),
                KeyCode::Numpad9 => Some(KeyInput::Numpad9),
                KeyCode::NumpadAdd => Some(KeyInput::NumpadAdd),
                KeyCode::NumpadSubtract => Some(KeyInput::NumpadSubtract),
                KeyCode::NumpadMultiply => Some(KeyInput::NumpadMultiply),
                KeyCode::NumpadDivide => Some(KeyInput::NumpadDivide),
                KeyCode::F1 => Some(KeyInput::F1),
                KeyCode::F2 => Some(KeyInput::F2),
                KeyCode::F3 => Some(KeyInput::F3),
                KeyCode::F4 => Some(KeyInput::F4),
                KeyCode::F5 => Some(KeyInput::F5),
                KeyCode::F6 => Some(KeyInput::F6),
                KeyCode::F7 => Some(KeyInput::F7),
                KeyCode::F8 => Some(KeyInput::F8),
                KeyCode::F9 => Some(KeyInput::F9),
                KeyCode::F10 => Some(KeyInput::F10),
                KeyCode::F11 => Some(KeyInput::F11),
                KeyCode::F12 => Some(KeyInput::F12),
                _ => None,
            },
        };

        if input_key.is_none() {
            warn!("key input not handled {event:?}");
            return;
        }

        let input_key = input_key.unwrap();
        if event.state.is_pressed() {
            self.press_key(input_key);
        } else {
            self.release_key(input_key);
        }
    }
}
