#![feature(test)]
extern crate test;
extern crate marksman_escape;

use test::{Bencher, black_box};
use marksman_escape::Escape;
use std::str::StrExt;


static MX_ESCAPES : &'static str = "a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@
a<cd>f{bcd}f<bcd>f{bc}e'ab\"d?f@b!d`f{bcd}f&b=d+f(bcd)f%bc$e!s[ytr]02\u{A0}a<cd>f{bcd}f&^<>@!+=@";

#[bench]
fn escape_mixed(b: &mut Bencher){
    b.bytes = MX_ESCAPES.len() as u64;
    b.iter(||{
        for _ in Escape::new(MX_ESCAPES.bytes()){
            black_box(());
        }
    });
}


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
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabcdefa
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
fn escape_no_spec(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in Escape::new(NO_ESCAPES.bytes()) {
            black_box(());
        }
    });
}


static ALL_ESCAPES_SHORT : &'static str = r##"<><><><><><><><><><><><><><><><><><><><><><><><><><><
><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>
><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><><>"##;

#[bench]
fn escape_spec_short(b: &mut Bencher){
    b.bytes = ALL_ESCAPES_SHORT.len() as u64;
    b.iter(||{
        for _ in Escape::new(ALL_ESCAPES_SHORT.bytes()) {
            black_box(());
        }
    });
}


static ALL_ESCAPES_LONG : &'static str = "[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}
[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]
[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]
[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]
[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]
[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]
[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]{}[]";

#[bench]
fn escape_spec_long(b: &mut Bencher){
    b.bytes = ALL_ESCAPES_LONG.len() as u64;
    b.iter(||{
        for _ in Escape::new(ALL_ESCAPES_LONG.bytes()) {
            black_box(());
        }
    });
}
