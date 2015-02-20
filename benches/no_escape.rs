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
fn no_escape_no_spec_chars(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in NO_ESCAPES.chars() {
            black_box(());
        }
    })
}

#[bench]
fn no_escape_map(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in NO_ESCAPES.chars().map(|_|{ '&' }) {
            black_box(());
        }
    })
}

#[bench]
fn no_escape_no_spec_bytes(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in NO_ESCAPES.bytes() {
            black_box(());
        }
    })
}

#[bench]
fn no_escape_no_spec_bytes_map(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in NO_ESCAPES.bytes().map(|_|{ 33 }) {
            black_box(());
        }
    })
}
