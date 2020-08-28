#![crate_name = "vertex_engine"]
#![crate_type = "lib"]
#![deny(missing_docs)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/VertexEngine/VertexEngine/master/assets/VertexEngine.png",
    html_logo_url = "https://raw.githubusercontent.com/VertexEngine/VertexEngine/master/assets/VertexEngine.png"
)]

//! A rust reimplementation of the [VertexEngine](https://github.com/VertexEngine/VertexEngine) game engine.

extern crate glfw;
extern crate vulkano;

pub mod core;
pub mod coregl;
pub mod prelude;
pub mod windowing;
