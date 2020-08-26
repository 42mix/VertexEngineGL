#![crate_name = "vertex_engine"]
#![crate_type = "lib"]
pub mod core;
pub mod windowing;

pub use crate::core::init;

#[cfg(test)]
mod tests {
    use crate::init;
    use crate::windowing::{WinMode, WindowProperties};

    #[test]
    fn it_works() {
        let mut ctx = init(WindowProperties::new(500, 500, "123", WinMode::Fullscreen)).unwrap();

        while !ctx.window_close_requested() {
            ctx.swap_buffers();
            ctx.poll_events();
        }
    }
}
