#![feature(unicode,core,collections)]
pub use escape::{Escape};
pub use unescape::{Unescape};
pub use unescape_named::{get_named_ref};

mod escape;
mod unescape;
mod unescape_named;
