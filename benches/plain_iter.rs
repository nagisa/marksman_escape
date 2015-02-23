#![feature(test)]
extern crate test;

use test::Bencher;
use test::black_box;

static NO_ESCAPES : &'static str = r##"abcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdef
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabc"##;

#[bench]
fn no_escape_chars(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in NO_ESCAPES.chars() {
            black_box(());
        }
    })
}

#[bench]
fn no_escape_bytes(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in NO_ESCAPES.bytes() {
            black_box(());
        }
    })
}

#[bench]
fn no_escape_bytes_filter(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        let mut iter = NO_ESCAPES.bytes();
        loop {
            match black_box(iter.next()) {
                Some(b'&') => 1,
                Some(_)    => 2,
                None       => break
            };
        }
    })
}
