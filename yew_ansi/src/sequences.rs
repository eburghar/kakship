use super::{
    cursor::CharCursor,
    graphic_rendition::{self, Sgr},
};

fn cursor_skip_space(cursor: &mut CharCursor) {
    // skip: !"#$%&'()*+,-./ (SPACE)
    cursor.read_while(|c| matches!(c, '\u{0020}'..='\u{002f}'));
}

/// ANSI Escape Sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Escape {
    Csi(Csi),
}
impl Escape {
    const ESC: char = '\u{001b}';

    fn parse(cursor: &mut CharCursor) -> Option<Self> {
        cursor.read_char(Self::ESC)?;
        cursor_skip_space(cursor);
        if Csi::peek(cursor) {
            Csi::parse(cursor).map(Self::Csi)
        } else {
            None
        }
    }
}

/// Control sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Csi {
    Sgr(Vec<Sgr>),
}
impl Csi {
    const START: char = '[';

    fn peek(cursor: &mut CharCursor) -> bool {
        cursor.peek_char(Self::START)
    }

    fn read_params<'a>(cursor: &mut CharCursor<'a>) -> Option<(char, Vec<&'a str>)> {
        let mut start = cursor.position();
        let mut end = start;
        let mut params = Vec::new();

        cursor.read_while(|c| {
            match c {
                ';' => {
                    params.push(start..end);
                    start = end + c.len_utf8();
                    end = start;
                    true
                }
                // 0–9:;<=>?
                '\u{0030}'..='\u{003f}' => {
                    end += c.len_utf8();
                    true
                }
                _ => false,
            }
        });

        if start != end {
            params.push(start..end);
        }

        cursor_skip_space(cursor);

        // read method name
        match cursor.read()? {
            c @ '\u{0040}'..='\u{007e}' => {
                let params = params.drain(..).map(|r| cursor.get(r).unwrap()).collect();
                Some((c, params))
            }
            _ => None,
        }
    }

    fn parse(cursor: &mut CharCursor) -> Option<Self> {
        cursor.read_char(Self::START)?;
        let (method, params) = Self::read_params(cursor)?;
        match method {
            'm' => {
                let params: Vec<usize> = params
                    .iter()
                    .map(|p| p.parse())
                    .collect::<Result<_, _>>()
                    .ok()?;
                let sgrs = graphic_rendition::parse_sgrs(params.iter().copied());
                Some(Self::Sgr(sgrs))
            }
            _ => None,
        }
    }
}

/// Read the next sequence in the given slice.
/// Returns the content before the escape sequence, the escape sequence itself, and everything following it.
/// The escape sequence can be `None` if it's invalid.
/// If the slice doesn't contain an escape sequence the entire string slice will be returned as the first item.
///
/// ```
/// # use yew_ansi::*;
/// let (pre, esc, post) = yew_ansi::read_next_sequence("Hello \u{001b}[32mWorld");
/// assert_eq!(pre, "Hello ");
/// assert_eq!(
///     esc,
///     Some(Escape::Csi(Csi::Sgr(vec![
///         Sgr::ColorFgName(ColorName::Green),
///     ])))
/// );
/// assert_eq!(post, "World");
/// ```
pub fn read_next_sequence(s: &str) -> (&str, Option<Escape>, &str) {
    s.find(Escape::ESC).map_or((s, None, ""), |index| {
        let (pre, post) = s.split_at(index);

        let mut cursor = CharCursor::new(post);
        let esc = Escape::parse(&mut cursor);

        (pre, esc, cursor.remainder())
    })
}

/// Parts of a string containing ANSI escape sequences.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Marker<'a> {
    /// Raw text without any escape sequences.
    Text(&'a str),
    /// Parsed escape sequence.
    Sequence(Escape),
}

/// Iterator yielding markers in a string.
///
/// Each item is a [`Marker`].
///
/// Returned by [`get_markers`].
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct MarkerIter<'a> {
    remaining: &'a str,
    buf: Option<Marker<'a>>,
}
impl<'a> MarkerIter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            remaining: s,
            buf: None,
        }
    }
}
impl<'a> Iterator for MarkerIter<'a> {
    type Item = Marker<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // handle the marker that might have been buffered by last iteration
        if let Some(marker) = self.buf.take() {
            return Some(marker);
        }

        while !self.remaining.is_empty() {
            let (pre, esc, post) = read_next_sequence(&self.remaining);
            self.remaining = post;

            let esc_marker = esc.map(Marker::Sequence);

            if pre.is_empty() {
                if let Some(marker) = esc_marker {
                    return Some(marker);
                }

                // nothing to yield right now, this either means we're at the end or we just skipped over an invalid escape sequence.
                // explicit "continue" here to make it clear.
                continue;
            } else {
                // store the escape code for the next iteration
                self.buf = esc_marker;
                return Some(Marker::Text(pre));
            }
        }

        None
    }
}

/// Iterate over all [`Marker`]s in given string.
///
/// ```
/// # use yew_ansi::*;
/// let markers = yew_ansi::get_markers("Hello \u{001b}[32mWorld\u{001b}[39;1m!").collect::<Vec<_>>();
/// assert_eq!(
///     markers,
///     vec![
///         Marker::Text("Hello "),
///         Marker::Sequence(Escape::Csi(Csi::Sgr(vec![
///             Sgr::ColorFgName(ColorName::Green),
///         ]))),
///         Marker::Text("World"),
///         Marker::Sequence(Escape::Csi(Csi::Sgr(vec![
///             Sgr::ResetColorFg,
///             Sgr::Bold,
///         ]))),
///         Marker::Text("!"),
///     ]
/// );
/// ```
pub fn get_markers(s: &str) -> MarkerIter {
    MarkerIter::new(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphic_rendition::ColorName;

    fn parse(s: &str) -> Option<Escape> {
        let s = s.replace("CSI ", "\u{001b} [");
        Escape::parse(&mut CharCursor::new(&s))
    }

    fn parse_sgr(s: &str) -> Vec<Sgr> {
        match parse(s) {
            Some(Escape::Csi(Csi::Sgr(sgr))) => sgr,
            _ => panic!("expected sgr"),
        }
    }

    #[test]
    fn parsing() {
        assert_eq!(
            parse_sgr("CSI 32 m"),
            vec![Sgr::ColorFgName(ColorName::Green)]
        );
        assert_eq!(
            parse_sgr("CSI 32;1m"),
            vec![Sgr::ColorFgName(ColorName::Green), Sgr::Bold]
        );
    }

    #[test]
    fn marking() {
        let markers = get_markers("Hello \u{001b} [33mWorld").collect::<Vec<_>>();
        assert_eq!(
            markers,
            vec![
                Marker::Text("Hello "),
                Marker::Sequence(Escape::Csi(Csi::Sgr(vec![Sgr::ColorFgName(
                    ColorName::Yellow
                )]))),
                Marker::Text("World"),
            ]
        )
    }
}
