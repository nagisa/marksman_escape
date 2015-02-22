#![feature(unicode,core,collections)]
pub use escape::*;
pub use unescape::*;
pub use unescape_named::*;

mod escape;
mod unescape;
mod unescape_named;
