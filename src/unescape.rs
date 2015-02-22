use std::iter::{Peekable, IntoIterator};

use std::str;
use std::char;
use std::num;

use unescape_named::{get_named_ref, LONGEST_NAMED_REFERENCE};


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
    inner: Peekable<I>,
    buffer_index: usize,
    decode_buffer: Vec<u8>,
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
            inner: i.peekable(),
            buffer_index: 0,
            decode_buffer: Vec::with_capacity(8)
        }
    }

    /// Replace buffer contents with character bytes
    fn buf_set_str(&mut self, string: &str) {
        self.decode_buffer.clear();
        self.decode_buffer.reserve(string.len());
        self.decode_buffer.push_all(string.as_bytes());
    }

    /// Push a character into buffer
    fn buf_push_char(&mut self, chr: char) {
        let mut buf: [u8; 4] = [0; 4];
        let size = chr.encode_utf8(&mut buf).unwrap();
        self.decode_buffer.extend(buf.into_iter().take(size).cloned());
    }

    /// Read a byte into buffer and return the read byte.
    fn next_through_buf(&mut self) -> Option<u8> {
        if let Some(x) = self.inner.next() {
            self.decode_buffer.push(x);
            Some(x)
        } else {
            None
        }
    }

    /// Read a byte from the buffer and increase buffer index.
    fn consume_buffer(&mut self) -> Option<u8> {
        if self.buffer_index < self.decode_buffer.len() {
            let r = unsafe { *(*self.decode_buffer).get_unchecked(self.buffer_index) };
            self.buffer_index += 1;
            Some(r)
        } else {
            None
        }
    }

    /// Convert a character reference number to unicode codepoint, replacing buffer contents,
    /// and return the first byte
    fn parse_codepoint(&mut self, codepoint: u32) -> u8 {
        match codepoint {
            0x80 => self.buf_set_str("\u{20AC}"),
            0x82 => self.buf_set_str("\u{201A}"),
            0x83 => self.buf_set_str("\u{0192}"),
            0x84 => self.buf_set_str("\u{201E}"),
            0x85 => self.buf_set_str("\u{2026}"),
            0x86 => self.buf_set_str("\u{2020}"),
            0x87 => self.buf_set_str("\u{2021}"),
            0x88 => self.buf_set_str("\u{02C6}"),
            0x89 => self.buf_set_str("\u{2030}"),
            0x8A => self.buf_set_str("\u{0160}"),
            0x8B => self.buf_set_str("\u{2039}"),
            0x8C => self.buf_set_str("\u{0152}"),
            0x8E => self.buf_set_str("\u{017D}"),
            0x91 => self.buf_set_str("\u{2018}"),
            0x92 => self.buf_set_str("\u{2019}"),
            0x93 => self.buf_set_str("\u{201C}"),
            0x94 => self.buf_set_str("\u{201D}"),
            0x95 => self.buf_set_str("\u{2022}"),
            0x96 => self.buf_set_str("\u{2013}"),
            0x97 => self.buf_set_str("\u{2014}"),
            0x98 => self.buf_set_str("\u{02DC}"),
            0x99 => self.buf_set_str("\u{2122}"),
            0x9A => self.buf_set_str("\u{0161}"),
            0x9B => self.buf_set_str("\u{203A}"),
            0x9C => self.buf_set_str("\u{0153}"),
            0x9E => self.buf_set_str("\u{017E}"),
            0x9F => self.buf_set_str("\u{0178}"),
            0x000B |
            0x0000...0x0008 |
            0x000D...0x001F |
            0x007F...0x009F |
            0xFDD0...0xFDEF |
            0xFFFE...0xFFFF |
            0x1FFFE...0x1FFFF |
            0x2FFFE...0x2FFFF |
            0x3FFFE...0x3FFFF |
            0x4FFFE...0x4FFFF |
            0x5FFFE...0x5FFFF |
            0x6FFFE...0x6FFFF |
            0x7FFFE...0x7FFFF |
            0x8FFFE...0x8FFFF |
            0x9FFFE...0x9FFFF |
            0xAFFFE...0xAFFFF |
            0xBFFFE...0xBFFFF |
            0xCFFFE...0xCFFFF |
            0xDFFFE...0xDFFFF |
            0xEFFFE...0xEFFFF |
            0xFFFFE...0xFFFFF => self.buf_set_str("\u{FFFD}"),
            x =>
                if let Some(c) = char::from_u32(x) {
                    self.decode_buffer.clear();
                    self.buf_push_char(c)
                } else {
                    self.buf_set_str("\u{FFFD}")
                },
        };
        self.consume_buffer().unwrap()
    }

    fn unescape_named(&mut self) -> u8 {
        loop {
            match self.next_through_buf() {
                Some(0x41...0x5A) | Some(0x61...0x7A) =>
                    // HTML5 specifies a few odd named character references that do not end in `;`.
                    // We have to check our named_ref table on every byte decoded. However this
                    // could also be a properly delimited character reference, so we must look
                    // ahead for `;`…
                    if let Some(0x3B) = self.inner.peek().cloned() {
                        continue
                    } else if let Some(string) = get_named_ref(&*self.decode_buffer) {
                        self.decode_buffer.clear();
                        self.decode_buffer.reserve(string.len());
                        self.decode_buffer.push_all(string);
                        return self.consume_buffer().unwrap();
                    } else {
                        continue
                    },
                Some(0x3B) => { // end of the character reference.
                    return if let Some(string) = get_named_ref(&*self.decode_buffer) {
                        self.decode_buffer.clear();
                        self.decode_buffer.reserve(string.len());
                        self.decode_buffer.push_all(string);
                        self.consume_buffer().unwrap()
                    } else {
                        0x26
                    }
                },
                _ => return 0x26 // not a valid named character reference
            }
        }
    }

    fn unescape_hex(&mut self) -> u8 {
        loop {
            match self.next_through_buf() {
                Some(0x30...0x39) | Some(0x41...0x46) | Some(0x61...0x66) => continue,
                Some(0x3B) => { // end of the character reference.
                    // Decode reference into bytes;
                    // Our buffer should look like this: #x1A3b5C;
                    // This means we have to discard bytes from both sides and parse that as
                    // hexadecimal integer.
                    let codepoint = match unsafe {
                        let buf = &self.decode_buffer[2..self.decode_buffer.len() - 1];
                        num::from_str_radix(str::from_utf8_unchecked(buf), 16)
                    } {
                        Ok(n) => n,
                        // TODO: emit \u{FFFD} on Overflow, Underflow and panic otherwise
                        // See rust-lang/rust#22639
                        // Err(Overflow/Underflow) => {
                        // self.buf_set_str("\u{FFFD}");
                        // return self.consume_buffer().unwrap();
                        // }
                        Err(e) => panic!("Could not decode parsed int: {:?}", e)
                    };
                    return self.parse_codepoint(codepoint);
                },
                _ => return 0x26 // not a valid hexadecimal character reference
            }

        }
    }

    fn unescape_dec(&mut self) -> u8 {
        loop {
            match self.next_through_buf() {
                Some(0x30...0x39) => continue, // just consuming
                Some(0x3B) => { // end of the character reference.
                    // Decode reference into bytes;
                    // Our buffer should look like this: #123456;
                    // This means we have to discard one byte from both sides and parse that as
                    // decimal integer.
                    let codepoint = match unsafe {
                        let buf = &self.decode_buffer[1..self.decode_buffer.len() - 1];
                        str::from_utf8_unchecked(buf).parse::<u32>()
                    } {
                        Ok(n) => n,
                        // TODO: emit \u{FFFD} on Overflow, Underflow and panic otherwise
                        // See rust-lang/rust#22639
                        // Err(Overflow/Underflow) => {
                        // self.buf_set_str("\u{FFFD}");
                        // return self.consume_buffer().unwrap();
                        // }
                        Err(e) => panic!("Could not decode parsed int: {:?}", e)
                    };
                    return self.parse_codepoint(codepoint);
                },
                _ => return 0x26 // not a valid decimal character reference
            }

        }
    }

    fn try_unescape(&mut self) -> u8 {
        self.decode_buffer.clear();
        self.buffer_index = 0;
        // We will fill buffer, try to decode character and return first byte
        // Note that at this point `&` should already be read from the iterator.
        match { self.inner.peek().cloned() } {
            // All these are not character references, return the already consumed `&`
            Some(0x09) => 0x26,
            Some(0x0A) => 0x26,
            Some(0x0C) => 0x26,
            Some(0x20) => 0x26,
            Some(0x22) => 0x26,
            Some(0x26) => 0x26,
            Some(0x27) => 0x26,
            Some(0x3C) => 0x26,
            Some(0x3E) => 0x26,
            None       => 0x26,
            // code made of either decimal or hexadecimal digits, maybe
            Some(0x23) => {
                self.next_through_buf();
                match { self.inner.peek().cloned() } {
                    Some(0x58) | Some(0x78) => {
                        self.next_through_buf();
                        self.unescape_hex()
                    }
                    Some(0x30...0x39) => self.unescape_dec(),
                    _                 => 0x26 // It wasn’t a valid escape after all
                }
            }
            // character references begin with ASCII letter
            Some(0x41...0x5A) | Some(0x61...0x7A) => {
                self.unescape_named()
            }
            _ => 0x26
        }
    }
}

impl<I: Iterator<Item=u8>> Iterator for Unescape<I> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if let r@Some(_) = self.consume_buffer() {
            return r;
        }
        if let Some(ch) = self.inner.next() {
            if ch == 0x26 { // We encountered an ampersand, this might be a character reference.
                Some(self.try_unescape())
            } else {
                Some(ch)
            }
        } else {
            None
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
        // run_test("&#1231231231231231232123123;", "�"); // REPLACEMENT CHARACTER intended here 😉
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
        // run_test("&#1231231231231231232123123;", "�"); // REPLACEMENT CHARACTER intended here 😉
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
