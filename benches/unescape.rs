#![feature(test)]
extern crate test;
extern crate marksman_escape;

use test::Bencher;
use test::black_box;
use marksman_escape::{Unescape, get_named_ref};
use std::str::StrExt;

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
fabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdefabcdeffabcdefabcdefabcdefabcdefabcdefabcdefabc"##;

#[bench]
fn unescape_no_spec(b: &mut Bencher){
    b.bytes = NO_ESCAPES.len() as u64;
    b.iter(||{
        for _ in Unescape::new(NO_ESCAPES.bytes()) {
            black_box(());
        }
    });
}



static ALL_ESCAPES_NAMED : &'static str = r##"&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp
&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp&lt;&gt;&amp;&lt&gt&amp"##;

#[bench]
fn unescape_spec_named(b: &mut Bencher){
    b.bytes = ALL_ESCAPES_NAMED.len() as u64;
    b.iter(||{
        for _ in Unescape::new(ALL_ESCAPES_NAMED.bytes()) {
            black_box(());
        }
    });
}



static ALL_ESCAPES_NUM : &'static str = r##"&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;
&#91;&#93;&#123;&#125;&#91;&#93;&#123;&#125;&#91;&#93;&#91;&#93;&#123;&#125;&#91;&#93;&#123;"##;

#[bench]
fn unescape_spec_num(b: &mut Bencher){
    b.bytes = ALL_ESCAPES_NUM.len() as u64;
    b.iter(||{
        for _ in Unescape::new(ALL_ESCAPES_NUM.bytes()) {
            black_box(());
        }
    });
}

static ALL_ESCAPES_HEX : &'static str = r##"&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;
&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5B;&#x5d;&#x7B;&#7d;&#x5B;&#x5d;&#x7B;"##;

#[bench]
fn unescape_spec_hex(b: &mut Bencher){
    b.bytes = ALL_ESCAPES_HEX.len() as u64;
    b.iter(||{
        for _ in Unescape::new(ALL_ESCAPES_HEX.bytes()) {
            black_box(());
        }
    });
}



#[bench]
fn named_ref_various(b: &mut Bencher){
    b.iter(||{
        black_box(get_named_ref(b"amp;").unwrap());
        black_box(get_named_ref(b"lt;").unwrap());
        black_box(get_named_ref(b"gt;").unwrap());
        black_box(get_named_ref(b"zwj;").unwrap());
        black_box(get_named_ref(b"ZeroWidthSpace;").unwrap());
        black_box(get_named_ref(b"Yacute;").unwrap());
    });
}

#[bench]
fn named_ref_simple_common(b: &mut Bencher){
    b.iter(||{
        black_box(get_named_ref(b"amp;").unwrap());
        black_box(get_named_ref(b"lt;").unwrap());
        black_box(get_named_ref(b"gt;").unwrap());
        black_box(get_named_ref(b"amp;").unwrap());
        black_box(get_named_ref(b"lt;").unwrap());
        black_box(get_named_ref(b"gt;").unwrap());
    });
}

#[bench]
fn named_ref_rare(b: &mut Bencher){
    b.iter(||{
        black_box(get_named_ref(b"zwj;").unwrap());
        black_box(get_named_ref(b"ZeroWidthSpace;").unwrap());
        black_box(get_named_ref(b"Yacute;").unwrap());
        black_box(get_named_ref(b"zwj;").unwrap());
        black_box(get_named_ref(b"ZeroWidthSpace;").unwrap());
        black_box(get_named_ref(b"Yacute;").unwrap());
    });
}
