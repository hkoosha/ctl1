#![allow(dead_code)]

use crossterm::event;
use std::fmt;

/// Represents an key.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum UiKey {
    /// Both Enter (or Return) and numpad Enter
    Enter,
    /// Tabulation key
    Tab,
    /// Backspace key
    Backspace,
    /// Escape key
    Esc,

    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,

    /// Insert key
    Ins,
    /// Delete key
    Delete,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,

    /// F0 key
    F0,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl UiKey {
    /// Returns the function key corresponding to the given number
    ///
    /// 1 -> F1, etc...
    ///
    /// # Panics
    ///
    /// If `n == 0 || n > 12`
    pub fn from_f(n: u8) -> UiKey {
        match n {
            0 => UiKey::F0,
            1 => UiKey::F1,
            2 => UiKey::F2,
            3 => UiKey::F3,
            4 => UiKey::F4,
            5 => UiKey::F5,
            6 => UiKey::F6,
            7 => UiKey::F7,
            8 => UiKey::F8,
            9 => UiKey::F9,
            10 => UiKey::F10,
            11 => UiKey::F11,
            12 => UiKey::F12,
            _ => panic!("unknown function key: F{}", n),
        }
    }
}

impl fmt::Display for UiKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UiKey::Alt(' ') => write!(f, "<Alt+Space>"),
            UiKey::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            UiKey::Char(' ') => write!(f, "<Space>"),
            UiKey::Alt(c) => write!(f, "<Alt+{}>", c),
            UiKey::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            UiKey::Char(c) => write!(f, "{}", c),
            UiKey::Left | UiKey::Right | UiKey::Up | UiKey::Down => write!(f, "<?? Arrow Key>"),
            UiKey::Enter
            | UiKey::Tab
            | UiKey::Backspace
            | UiKey::Esc
            | UiKey::Ins
            | UiKey::Delete
            | UiKey::Home
            | UiKey::End
            | UiKey::PageUp
            | UiKey::PageDown => write!(f, "<??>"),
            _ => write!(f, "??"),
        }
    }
}

impl From<event::KeyEvent> for UiKey {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => UiKey::Esc,
            event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => UiKey::Backspace,
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => UiKey::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => UiKey::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => UiKey::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => UiKey::Down,
            event::KeyEvent {
                code: event::KeyCode::Home,
                ..
            } => UiKey::Home,
            event::KeyEvent {
                code: event::KeyCode::End,
                ..
            } => UiKey::End,
            event::KeyEvent {
                code: event::KeyCode::PageUp,
                ..
            } => UiKey::PageUp,
            event::KeyEvent {
                code: event::KeyCode::PageDown,
                ..
            } => UiKey::PageDown,
            event::KeyEvent {
                code: event::KeyCode::Delete,
                ..
            } => UiKey::Delete,
            event::KeyEvent {
                code: event::KeyCode::Insert,
                ..
            } => UiKey::Ins,
            event::KeyEvent {
                code: event::KeyCode::F(n),
                ..
            } => UiKey::from_f(n),
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => UiKey::Enter,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => UiKey::Tab,

            // First check for char + modifier
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::ALT, ..
            } => UiKey::Alt(c),
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL, ..
            } => UiKey::Ctrl(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => UiKey::Char(c),

            _ => UiKey::Unknown,
        }
    }
}