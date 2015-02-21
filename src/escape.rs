use std::num::Int;

const LONGEST_ESCAPE: usize = 6;
static ZERO : [u8; LONGEST_ESCAPE] = [0; LONGEST_ESCAPE];
// Basic set of escapes: &, <, >, "
static AMP  : [u8; LONGEST_ESCAPE] = [0x61, 0x6d, 0x70, 0x3b, 0, 0];
static LT   : [u8; LONGEST_ESCAPE] = [0x6c, 0x74, 0x3b, 0, 0, 0];
static GT   : [u8; LONGEST_ESCAPE] = [0x67, 0x74, 0x3b, 0, 0, 0];
static QUOT : [u8; LONGEST_ESCAPE] = [0x23, 0x33, 0x34, 0x3b, 0, 0];
// Less basic set, but important nevertheless: ' and `
// Grave is supposedly allowed in IE as attribute value wrapper.
static APOS : [u8; LONGEST_ESCAPE] = [0x23, 0x33, 0x39, 0x3b, 0, 0];
static GRV  : [u8; LONGEST_ESCAPE] = [0x23, 0x39, 0x36, 0x3b, 0, 0];
// Now these only matter in cases where attributes are not quoted.
static BANG : [u8; LONGEST_ESCAPE] = [0x23, 0x33, 0x33, 0x3b, 0, 0];
static USD  : [u8; LONGEST_ESCAPE] = [0x23, 0x33, 0x36, 0x3b, 0, 0];
static PERC : [u8; LONGEST_ESCAPE] = [0x23, 0x33, 0x37, 0x3b, 0, 0];
static LPR  : [u8; LONGEST_ESCAPE] = [0x23, 0x34, 0x30, 0x3b, 0, 0];
static RPR  : [u8; LONGEST_ESCAPE] = [0x23, 0x34, 0x31, 0x3b, 0, 0];
static PLUS : [u8; LONGEST_ESCAPE] = [0x23, 0x34, 0x33, 0x3b, 0, 0];
static EQ   : [u8; LONGEST_ESCAPE] = [0x23, 0x36, 0x31, 0x3b, 0, 0];
static AT   : [u8; LONGEST_ESCAPE] = [0x23, 0x36, 0x34, 0x3b, 0, 0];
static LBR  : [u8; LONGEST_ESCAPE] = [0x23, 0x39, 0x31, 0x3b, 0, 0];
static RBR  : [u8; LONGEST_ESCAPE] = [0x23, 0x39, 0x33, 0x3b, 0, 0];
static LBRK : [u8; LONGEST_ESCAPE] = [0x23, 0x31, 0x32, 0x33, 0x3b, 0];
static RBRK : [u8; LONGEST_ESCAPE] = [0x23, 0x31, 0x32, 0x35, 0x3b, 0];

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
    /// Create a wrapper iterator which will escape all the bytes of internal iterator.
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
            idx: 0,
            inner_buffer: &ZERO
        }
    }

    #[inline(always)]
    fn buf(&mut self, escape: &'static [u8; LONGEST_ESCAPE]) -> Option<u8> {
        self.idx = 0;
        self.inner_buffer = escape;
        Some(0x26)
    }
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
                0x26     => self.buf(&AMP),
                0x3E     => self.buf(&GT),
                0x3C     => self.buf(&LT),
                0x21     => self.buf(&BANG),
                0x22     => self.buf(&QUOT),
                0x24     => self.buf(&USD),
                0x25     => self.buf(&PERC),
                0x27     => self.buf(&APOS),
                0x28     => self.buf(&LPR),
                0x29     => self.buf(&RPR),
                0x2B     => self.buf(&PLUS),
                0x3D     => self.buf(&EQ),
                0x40     => self.buf(&AT),
                0x5B     => self.buf(&LBR),
                0x5D     => self.buf(&RBR),
                0x60     => self.buf(&GRV),
                0x7B     => self.buf(&LBRK),
                0x7D     => self.buf(&RBRK),
                _        => Some(ch)
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
