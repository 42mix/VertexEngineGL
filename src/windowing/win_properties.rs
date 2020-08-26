pub struct WindowProperties<'title> {
    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) title: &'title str,
    pub(crate) win_mode: WinMode,
}

impl<'title> WindowProperties<'title> {
    pub fn new(width: u32, height: u32, title: &'title str, win_mode: WinMode) -> Self {
        Self {
            width,
            height,
            title,
            win_mode,
        }
    }
}

pub enum WinMode {
    Windowed,
    Fullscreen,
}
