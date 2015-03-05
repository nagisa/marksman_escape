use std::char;

use unescape_named::LONGEST_NAMED_REFERENCE;


/// Unescape a HTML-encoded stream of bytes.
///
/// The [HTML5 named character references][html5-nref] (`&amp;`), decimal character references
/// (`&#123;`) and hexadecimal character references (`&#x1BA;`) are supported.
///
/// The implementation works with bytes interpreting them to be ASCII, which means that any
/// ASCII-compatible encoding, including UTF-8, is supported.
///
/// [html5-nref]: http://www.w3.org/TR/html5/syntax.html#named-character-references
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Unescape<I: Iterator<Item=u8>>{
    inner: I,
    index: usize, // Index into the buffer
    buffer: Vec<I::Item>,
}


impl<I: Iterator<Item=u8>> Unescape<I> {
    /// Create an iterator adaptor which will unescape all the character references found in the
    /// internal iterator.
    ///
    /// # Usage
    ///
    /// ```
    /// use marksman_escape::Unescape;
    /// let string = "&lt;hello&gt;&amp;world&#60;/hello&#x3e;";
    /// let unescaped = String::from_utf8(Unescape::new(string.bytes()).collect()).unwrap();
    /// assert_eq!("<hello>&world</hello>", &*unescaped);
    /// ```
    pub fn new(i: I) -> Unescape<I> {
        Unescape {
            inner: i,
            index: 0,
            buffer: Vec::with_capacity(8)
        }
    }

    /// Unsafely read a character from the buffer
    #[inline]
    fn unext(&mut self) -> u8 {
        let r = unsafe { *(*self.buffer).get_unchecked(self.index) };
        self.index += 1;
        r
    }

    /// Drop the contents from the buffer.
    #[inline]
    fn drop_buffer(&mut self) {
        unsafe {
            self.buffer.set_len(0);
        }
    }

    /// Read the next character from internal iterator adding the byte into buffer.
    #[inline]
    fn read_to_buffer(&mut self) -> Option<u8> {
        if let Some(x) = self.inner.next() {
            self.buffer.push(x);
            Some(x)
        } else {
            None
        }
    }

    fn from_codepoint(&mut self, codepoint: u32) -> u8 {
        self.drop_buffer();
        self.index = 0;
        let string = if codepoint & 0xfffe == 0xfffe {
            "\u{FFFD}"
        } else {
            match codepoint {
                0x80 => "\u{20AC}",
                0x82 => "\u{201A}",
                0x83 => "\u{0192}",
                0x84 => "\u{201E}",
                0x85 => "\u{2026}",
                0x86 => "\u{2020}",
                0x87 => "\u{2021}",
                0x88 => "\u{02C6}",
                0x89 => "\u{2030}",
                0x8A => "\u{0160}",
                0x8B => "\u{2039}",
                0x8C => "\u{0152}",
                0x8E => "\u{017D}",
                0x91 => "\u{2018}",
                0x92 => "\u{2019}",
                0x93 => "\u{201C}",
                0x94 => "\u{201D}",
                0x95 => "\u{2022}",
                0x96 => "\u{2013}",
                0x97 => "\u{2014}",
                0x98 => "\u{02DC}",
                0x99 => "\u{2122}",
                0x9A => "\u{0161}",
                0x9B => "\u{203A}",
                0x9C => "\u{0153}",
                0x9E => "\u{017E}",
                0x9F => "\u{0178}",
                0x000B
                | 0x0000...0x0008
                | 0x000D...0x001F
                | 0x007F...0x009F
                | 0xFDD0...0xFDEF => "\u{FFFD}",
                chr  => if let Some(chr) = char::from_u32(chr) {
                    unsafe {
                        self.buffer.set_len(4);
                        let len = chr.encode_utf8(&mut self.buffer[..]).unwrap();
                        self.buffer.set_len(len);
                        return self.unext();
                    }
                } else {
                    "\u{FFFD}"
                }
            }
        };
        self.buffer.push_all(string.as_bytes());
        self.unext()
    }

    fn unescape_named(&mut self, byte: u8) -> u8 {
        use unescape_named::Matcher;
        use unescape_named::RefMatch::*;
        let mut matcher = Matcher::new();
        matcher.feed_byte(byte);
        loop {
            match self.read_to_buffer() {
                Some(b';') => match matcher.feed_byte(b';') {
                    Match(m) => {
                        self.drop_buffer();
                        self.buffer.push_all(m);
                        return self.unext();
                    },
                    _ => return b'&'
                },
                Some(b@b'a'...b'z') | Some(b@b'A'...b'Z') | Some(b@b'0'...b'9') => {
                    match matcher.feed_byte(b) {
                        Mismatch   => return b'&',
                        Partial => continue,
                        Match(m) => {
                            self.drop_buffer();
                            self.buffer.push_all(m);
                            // Check for ; next
                            let oldl = self.buffer.len();
                            if let Some(b';') = self.read_to_buffer() {
                                // If we read the `;`, just get rid of it.
                                unsafe { self.buffer.set_len(oldl) }
                            }
                            return self.unext();
                        }
                    }
                },
                _ => return b'&'
            }
        }
    }

    // TODO: this can be abused to store a lot of characters in memory by providing it
    // &#<very very long sequence of decimal digits>
    fn unescape_dec(&mut self, byte: u8) -> u8 {
        let mut value: u32 = (byte - b'0') as u32;
        loop {
            match self.read_to_buffer() {
                Some(b';') => {// end of a character reference with a valid syntax
                    return self.from_codepoint(value);
                },
                Some(b@b'0'...b'9') => if value <= 0x10FFFF {
                    value = (value * 10) + ((b - b'0') as u32);
                },
                _ => return b'&' // not an escape
            }
        }
    }

    // TODO: this can be abused to store a lot of characters in memory by providing it
    // &#<very very long sequence of hex digits>
    fn unescape_hex(&mut self) -> u8 {
        let mut value: u32 = 0;
        loop {
            let byte = self.read_to_buffer();
            if let Some(b';') = byte {
                return self.from_codepoint(value);
            } else if let Some(b@b'0'...b'9') = byte {
                if value <= 0x10FFFF {
                    value = (value * 0x10) + ((b - b'0') as u32);
                }
            } else if let Some(b@b'a'...b'f') = byte.map(|x| { x | 0b0010_0000}) {
                if value <= 0x10FFFF {
                    value = (value * 16) + ((b - b'a' + 10) as u32);
                }
            } else {
                return b'&'; // Not a valid escape sequence
            }
        }
    }

    #[inline]
    fn unescape_numerical(&mut self) -> u8 {
        match self.read_to_buffer() {
            Some(b'x') | Some(b'X') => self.unescape_hex(),
            Some(c@b'0'...b'9')     => self.unescape_dec(c),
            _                       => b'&' // not an escape, return the consumed ampersand
        }
    }

    #[inline]
    fn unescape(&mut self) -> u8 {
        self.drop_buffer();
        self.index = 0;

        match self.read_to_buffer() {
            Some(b'#') => self.unescape_numerical(),
            Some(c@b'a'...b'z') | Some(c@b'A'...b'Z') => self.unescape_named(c),
            _ => b'&' // not an escape, return the consumed ampersand
        }
    }
}

impl<I: Iterator<Item=u8>> Iterator for Unescape<I> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        match {if self.index < self.buffer.len() {Some(self.unext())} else {self.inner.next()}} {
            Some(b'&') => Some(self.unescape()),
            r@_        => r
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, u) = self.inner.size_hint();
        // No numerical reference can be longer than 32, which is the length of longest known
        // named reference.
        (l / LONGEST_NAMED_REFERENCE, u)
    }
}


#[cfg(test)]
mod test {
    use std::str;
    use ::Unescape;
    fn run_test(from: &str, to: &str) {
        let dv = Unescape::new(from.bytes()).collect::<Vec<_>>();
        let d = str::from_utf8(&*dv).unwrap();
        assert_eq!(d, to);
    }
    #[test]
    fn no_escapes(){
        run_test("Hello world!", "Hello world!");
    }

    #[test]
    fn dec_escape(){
        run_test("&#38;", "&");
        run_test("&#62;", ">");
        run_test("&#60;", "<");
        run_test("&#33;", "!");
        run_test("&#&#33;", "&#!");
        run_test("&#;&#33;", "&#;!");
        run_test("&#12345$", "&#12345$");
        run_test("&#0;&#33;", "�!"); // REPLACEMENT CHARACTER intended here 😉
        run_test("&#11822;&#33;", "⸮!");
        run_test("&#65533;", "�"); // REPLACEMENT CHARACTER intended here 😉
        run_test("&#1234567890;", "�"); // REPLACEMENT CHARACTER intended here 😉
    }

    #[test]
    fn hex_escape(){
        run_test("&#x26;", "&");
        run_test("&#x3E;", ">");
        run_test("&#x3e;", ">");
        run_test("&#x3C;", "<");
        run_test("&#x3c;", "<");
        run_test("&#x21;", "!");
        run_test("&#&#x21;", "&#!");
        run_test("&#;&#x21;", "&#;!");
        run_test("&#x12345$", "&#x12345$");
        run_test("&#x0;&#x21;", "�!"); // REPLACEMENT CHARACTER intended here 😉
        run_test("&#x2e2E;&#x21;", "⸮!");
        run_test("&#x2E2e;&#x21;", "⸮!");
        run_test("&#xfffd;", "�"); // REPLACEMENT CHARACTER intended here 😉
        run_test("&#x1234567890ABCDEF;", "�"); // REPLACEMENT CHARACTER intended here 😉
    }

    #[test]
    fn named_escape(){
        run_test("&amp;", "&");
        run_test("&AMP;", "&");
        run_test("&AMP", "&");
        run_test("&AmP;", "&AmP;");
        run_test("&gt;", ">");
        run_test("&Gt;", "\u{226b}");
        run_test("&lt;", "<");
        run_test("&LT", "<");
        run_test("&excl;", "!");
        run_test("&&excl;", "&!");
        run_test("&;&excl;", "&;!");
        run_test("&12345;", "&12345;");
        run_test("&UnderParenthesis;", "⏝");
        run_test("&underParenthesis;", "&underParenthesis;");
        run_test("&Underparenthesis;", "&Underparenthesis;");
    }
}
