#![feature(test)]
extern crate test;
extern crate marksman_escape;

use test::Bencher;
use test::black_box;
use marksman_escape::Escaped;

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
        for _ in Escaped::new(MX_ESCAPES) {
            black_box(());
        }
    });
}
