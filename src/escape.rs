#[derive(PartialEq, Debug)]
pub enum Token<'a> {
	Percent,
	Str(&'a str),
}

pub struct EscapeIterator<'a> {
	remainder: &'a str,
}

impl<'a> EscapeIterator<'a> {
	pub fn new(string: &'a str) -> Self {
		Self { remainder: string }
	}
}

/// Iterator that either return a percent which is not prefixing opt{, var{, sh, or { (as they are reserved as expansion
/// expressions in kakoune) or a string slice without percent
impl<'a> Iterator for EscapeIterator<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut cursor = 0;
		loop {
			if self.remainder.is_empty() {
				return None;
			} else if let Some(end) = self.remainder[cursor..].find("%") {
				let chunk = &self.remainder[..cursor + end];
				let remainder = &self.remainder[cursor + end..];
				if remainder.starts_with("%val{") || remainder.starts_with("%opt{") {
					cursor += end + 5;
					continue;
				} else if remainder.starts_with("%sh{") {
					cursor += end + 4;
					continue;
				} else if remainder.starts_with("%{") {
					cursor += end + 2;
					continue;
				} else if cursor + end == 0 {
					self.remainder = &self.remainder[1..];
					return Some(Token::Percent);
				} else {
					self.remainder = &self.remainder[cursor + end..];
					return Some(Token::Str(chunk));
				}
			} else {
				let chunk = self.remainder;
				self.remainder = "";
				return Some(Token::Str(chunk));
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_empty() {
		let tokens: Vec<_> = EscapeIterator::new("").collect();
		assert_eq!(tokens, &[]);
	}

	#[test]
	fn test_percent() {
		let tokens: Vec<_> = EscapeIterator::new("%").collect();
		assert_eq!(tokens, vec![Token::Percent]);
	}

	#[test]
	fn test_opt() {
		let tokens: Vec<_> = EscapeIterator::new("%opt{session}").collect();
		assert_eq!(tokens, vec![Token::Str("%opt{session}")]);
	}

	#[test]
	fn test_shell() {
		let tokens: Vec<_> = EscapeIterator::new("%sh{basename \"$kak_file\"}").collect();
		assert_eq!(tokens, vec![Token::Str("%sh{basename \"$kak_file\"}")]);
	}

	#[test]
	fn test_expansion() {
		let tokens: Vec<_> = EscapeIterator::new("%{echo session}").collect();
		assert_eq!(tokens, vec![Token::Str("%{echo session}")]);
	}

	#[test]
	fn test_mixed() {
		let tokens: Vec<_> = EscapeIterator::new("98% %val{session}").collect();
		assert_eq!(
			tokens,
			vec![
				Token::Str("98"),
				Token::Percent,
				Token::Str(" %val{session}")
			]
		);
	}
}
