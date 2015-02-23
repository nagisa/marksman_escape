use std::num::Int;

const LONGEST_ESCAPE : usize = 6;

/// Escape a byte stream with HTML-encoded variants of certain characters.
///
/// Current implementation escapes the following list of characters:
///
/// <table>
///     <tr>
///         <td>&amp;</td>
///         <td>&lt;</td>
///         <td>&gt;</td>
///         <td>"</td>
///         <td>'</td>
///         <td>`</td>
///         <td>!</td>
///         <td>$</td>
///         <td>%</td>
///     </tr><tr>
///         <td>(</td>
///         <td>)</td>
///         <td>+</td>
///         <td>=</td>
///         <td>@</td>
///         <td>[</td>
///         <td>]</td>
///         <td>{</td>
///         <td>}</td>
///     </tr>
/// </table>
///
/// This list was built having in mind good XSS protection first and performance second. Note, that
/// the list of escaped characters is not a part of stable interface and might change between
/// releases.
///
/// The implementation works with bytes interpreting them to be ASCII, which means that any
/// ASCII-compatible encoding, including UTF-8, is supported.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Escape<I: Iterator<Item=u8>> {
    inner: I,
    idx: usize,
    inner_buffer: &'static [u8; LONGEST_ESCAPE]
}


impl<I: Iterator<Item=u8>> Escape<I> {
    /// Create an iterator adaptor which will escape all the bytes of internal iterator.
    ///
    /// # Usage
    ///
    /// ```
    /// use marksman_escape::Escape;
    /// let string = "<hello>&world</hello>";
    /// let escaped = String::from_utf8(Escape::new(string.bytes()).collect()).unwrap();
    /// assert_eq!("&lt;hello&gt;&amp;world&lt;/hello&gt;", &*escaped);
    /// ```
    pub fn new(i: I) -> Escape<I> {
        static ZERO : [u8; LONGEST_ESCAPE] = [0; LONGEST_ESCAPE];
        Escape {
            inner: i,
            idx: 0,
            inner_buffer: &ZERO
        }
    }
}

macro_rules! set_buf {
    ($o:expr, $x:expr) => {{
        static ESCAPE : [u8; LONGEST_ESCAPE] = $x;
        $o.idx = 0;
        $o.inner_buffer = &ESCAPE;
        Some(b'&')
    }}
}

impl<I: Iterator<Item=u8>> Iterator for Escape<I> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        unsafe {
            if *self.inner_buffer.get_unchecked(self.idx) != 0 {
                let r = *self.inner_buffer.get_unchecked(self.idx);
                self.idx += 1;
                return Some(r);
            }
        }
        if let Some(ch) = self.inner.next() {
            match ch {
                // Basic escapes
                b'&'  => set_buf!(self, [b'a', b'm', b'p', b';', 0, 0]),
                b'>'  => set_buf!(self, [b'g', b't', b';', 0, 0, 0]),
                b'<'  => set_buf!(self, [b'l', b't', b';', 0, 0, 0]),
                b'"'  => set_buf!(self, [b'#', b'3', b'4', b';', 0, 0]),
                b'\'' => set_buf!(self, [b'#', b'3', b'9', b';', 0, 0]),
                b'`'  => set_buf!(self, [b'#', b'9', b'6', b';', 0, 0]),
                // These only matter in cases where attributes are not quoted.
                b'!'  => set_buf!(self, [b'#', b'3', b'3', b';', 0, 0]),
                b'$'  => set_buf!(self, [b'#', b'3', b'6', b';', 0, 0]),
                b'%'  => set_buf!(self, [b'#', b'3', b'7', b';', 0, 0]),
                b'('  => set_buf!(self, [b'#', b'4', b'0', b';', 0, 0]),
                b')'  => set_buf!(self, [b'#', b'4', b'1', b';', 0, 0]),
                b'+'  => set_buf!(self, [b'#', b'4', b'3', b';', 0, 0]),
                b'='  => set_buf!(self, [b'#', b'6', b'1', b';', 0, 0]),
                b'@'  => set_buf!(self, [b'#', b'6', b'4', b';', 0, 0]),
                b'['  => set_buf!(self, [b'#', b'9', b'1', b';', 0, 0]),
                b']'  => set_buf!(self, [b'#', b'9', b'3', b';', 0, 0]),
                b'{'  => set_buf!(self, [b'#', b'1', b'2', b'3', b';', 0]),
                b'}'  => set_buf!(self, [b'#', b'1', b'2', b'5', b';', 0]),
                _     => Some(ch)
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, u) = self.inner.size_hint();
        (l, if let Some(u_) = u {u_.checked_mul(LONGEST_ESCAPE)} else {None})
    }
}
