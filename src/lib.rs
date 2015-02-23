//! A fastish HTML-encoding and HTML-decoding implementation in Rust.
//!
//! ### Fast…ish?
//!
//! The thing is that I only really compared the implementation against Python’s `html.encode` and
//! `html.decode` and Python’s numbers are so abysmal there’s nothing to compare…
//!
//! Anyway, here’s the (totally unscientific) numbers on Intel’s i7-4750HQ@2.00GHz and explanation
//! of them:
//!
//! ```
//! test no_escape_no_spec_bytes     ... bench:      1153 ns/iter (+/- 6) = 915 MB/s
//! test no_escape_no_spec_chars     ... bench:      2486 ns/iter (+/- 14) = 424 MB/s
//! ```
//!
//! Above is the speed at which iterators of bytes and unicode characters, respectively, can be
//! consumed without doing anything else. This serves as a benchmark anchor to compare with, so it
//! is clear how much overhead escaping and unescaping introduce.
//!
//! ```
//! test escape_mixed            ... bench:      8825 ns/iter (+/- 357) = 111 MB/s
//! test escape_no_spec          ... bench:      8348 ns/iter (+/- 315) = 270 MB/s
//! test escape_spec_long        ... bench:      8880 ns/iter (+/- 464) = 73 MB/s
//! test escape_spec_short       ... bench:      6756 ns/iter (+/- 330) = 96 MB/s
//!
//! test unescape_no_spec        ... bench:      8052 ns/iter (+/- 291) = 218 MB/s
//! test unescape_spec_hex       ... bench:      5692 ns/iter (+/- 230) = 147 MB/s
//! test unescape_spec_named     ... bench:      8582 ns/iter (+/- 360) = 102 MB/s
//! test unescape_spec_num       ... bench:      6078 ns/iter (+/- 240) = 138 MB/s
//! ```
//!
//! Both escape and unescape benchmarks test how fast the input is consumed, rather than produced,
//! so the numbers are expected. They are likely to improve further as codegen for `Iterator`s is
//! improved and my own battles against LLVM are concluded.
#![feature(unicode,core,collections)]

pub use escape::{Escape};
pub use unescape::{Unescape};
pub use unescape_named::{get_named_ref};

mod escape;
mod unescape;
mod unescape_named;
