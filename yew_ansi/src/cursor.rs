use std::slice::SliceIndex;

/// Cursor for characters in a string slice.
#[derive(Clone, Debug)]
pub struct CharCursor<'a> {
    s: &'a str,
    pos: usize,
}
impl<'a> CharCursor<'a> {
    /// Create a cursor for the given slice.
    /// The cursor starts at the start of the slice.
    pub fn new(s: &'a str) -> Self {
        Self { s, pos: 0 }
    }

    /// Get the cursor's position in the slice.
    /// This will always point to a valid char boundary.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Index the contained string slice.
    /// The index isn't bound by the cursor's position, only by the underlying slice.
    pub fn get<I: SliceIndex<str>>(&self, i: I) -> Option<&'a I::Output> {
        self.s.get(i)
    }

    /// Get a string slice containing all the characters following the current position.
    pub fn remainder(&self) -> &'a str {
        // SAFETY: pos always points to a char boundary
        unsafe { self.s.get_unchecked(self.pos..) }
    }

    fn peek(&self) -> Option<char> {
        self.remainder().chars().next()
    }

    /// Check if the next character matches the expected one.
    /// This doesn't advance the cursor's position.
    pub fn peek_char(&self, expected: char) -> bool {
        self.peek() == Some(expected)
    }

    fn read_if(&mut self, f: impl FnOnce(char) -> bool) -> Option<char> {
        match self.peek() {
            Some(c) if f(c) => {
                self.pos += c.len_utf8();
                Some(c)
            }
            _ => None,
        }
    }

    /// Read the next character and advance the cursor.
    pub fn read(&mut self) -> Option<char> {
        self.read_if(|_| true)
    }

    /// Read the next character and advance the cursor only if it matches the expected one.
    /// Returns `None` if the character didn't match.
    pub fn read_char(&mut self, expected: char) -> Option<char> {
        self.read_if(|c| c == expected)
    }

    /// Advance the cursor while the given function returns `true`.
    /// The cursor won't advance if the function returns `false`.
    pub fn read_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while self.read_if(&mut f).is_some() {}
    }
}
