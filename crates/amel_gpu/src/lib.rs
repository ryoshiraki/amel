#![warn(clippy::all)]

pub use wgpu;

// pub mod bind;
// pub mod blend;
// pub mod buffer;
// pub mod pass;
// pub mod pipeline;
// pub mod shader;
// pub mod texture;

pub mod prelude {
    // pub use super::bind::prelude::*;
    // pub use super::blend::*;
    // pub use super::buffer::*;
    // pub use super::pass::*;
    // pub use super::pipeline::prelude::*;
    // pub use super::shader::*;
    // pub use super::texture::prelude::*;
    // pub use super::texture::*;

    pub use super::wgpu;
}
