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
    buffer: u64
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
        Escape {
            inner: i,
            buffer: 0
        }
    }
}

impl<I: Iterator<Item=u8>> Iterator for Escape<I> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.buffer != 0 {
            let ret = Some(self.buffer as u8);
            self.buffer >>= 8;
            ret
        } else if let Some(ch) = self.inner.next() {
            self.buffer = match ch {
                // Basic escapes
                b'&'  => 0x3b_70_6d_61,    // amp;
                b'>'  => 0x3b_74_67,       // gt;
                b'<'  => 0x3b_74_6c,       // lt;
                b'"'  => 0x3b_34_33_23,    // #34;
                b'\'' => 0x3b_39_33_23,    // #39;
                b'`'  => 0x3b_36_39_23,    // #96;
                // These only matter in cases where attributes are not quoted.
                b'!'  => 0x3b_33_33_23,    // #33;
                b'$'  => 0x3b_36_33_23,    // #36;
                b'%'  => 0x3b_37_33_23,    // #37;
                b'('  => 0x3b_30_34_23,    // #40;
                b')'  => 0x3b_31_34_23,    // #41;
                b'+'  => 0x3b_33_34_23,    // #43;
                b'='  => 0x3b_31_36_23,    // #61;
                b'@'  => 0x3b_34_36_23,    // #64;
                b'['  => 0x3b_31_39_23,    // #91;
                b']'  => 0x3b_33_39_23,    // #93;
                b'{'  => 0x3b_33_32_31_23, // #123;
                b'}'  => 0x3b_35_32_31_23, // #125;
                _     => return Some(ch)
            };
            Some(b'&')
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, u) = self.inner.size_hint();
        (l, if let Some(u_) = u {u_.checked_mul(LONGEST_ESCAPE)} else {None})
    }
}
