/// The properties to open the window with.
pub struct WindowProperties<'title> {
    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) title: &'title str,
    pub(crate) win_mode: WinMode,
}

impl<'title> WindowProperties<'title> {
    /// Create a WindowProperties object, given the fields
    pub fn new(width: u32, height: u32, title: &'title str, win_mode: WinMode) -> Self {
        Self {
            width,
            height,
            title,
            win_mode,
        }
    }
}

/// The window mode to use for creating the window
pub enum WinMode {
    /// As a "windowed" window
    Windowed,
    /// As a fullscreen window
    Fullscreen,
}
