pub enum RefMatch {
    Match(&'static [u8]),
    Partial,
    Mismatch
}

pub struct Trie(&'static [Option<&'static Trie>], u16, u8);
static NULL_TRIE: Trie = Trie(&[], !0, !0);

pub struct Matcher(&'static Trie);

impl Matcher {
    #[inline]
    pub fn new() -> Matcher {
        Matcher(&TRIE)
    }

    #[inline]
    pub fn feed_byte(&mut self, byte: u8) -> RefMatch {
        let idx = (byte as isize - (self.0).2 as isize) as usize; // wrapping is fine
        self.0 = if let Some(Some(el)) = (self.0).0.get(idx).cloned() {
            el
        } else {
            self.0 = &NULL_TRIE;
            return RefMatch::Mismatch;
        };
        if (self.0).1 == !0 {
            RefMatch::Partial
        } else {
            RefMatch::Match(unsafe { DECODED.get_unchecked((self.0).1 as usize) })
        }
    }
}

#[inline]
pub fn get_named_ref(name: &[u8]) -> Option<&'static [u8]> {
    let mut matcher = Matcher::new();
    let mut r = RefMatch::Mismatch;
    for i in name {
        r = matcher.feed_byte(*i);
    }
    if let RefMatch::Match(m) = r {
        Some(m)
    } else {
        None
    }
}

include!("unescape_named_gen.rs");
