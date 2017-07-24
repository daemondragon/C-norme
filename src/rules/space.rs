use rules::Rule;

pub struct LineSize {
	max: usize
}

impl LineSize {
	pub fn new(max_line_size: usize) -> LineSize {
		LineSize { max: max_line_size }
	}
}

impl Rule for LineSize {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.len() >= self.max {
				errors.push(format!("[{}:{}]Line size exceeded.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct SpaceIndentation {

}

impl SpaceIndentation {
	pub fn new() -> SpaceIndentation {
		SpaceIndentation { }
	}
}

impl Rule for SpaceIndentation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.contains("\t") {
				errors.push(format!("[{}:{}]Tab used instead of spaces.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct TrailingWhiteSpace {

}

impl TrailingWhiteSpace {
	pub fn new() -> TrailingWhiteSpace {
		TrailingWhiteSpace { }
	}
}

impl Rule for TrailingWhiteSpace {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			match line.chars().last() {
				Some(last_c) if last_c.is_whitespace() => errors.push(format!("[{}:{}]Trailing whitespace.",filename, line_number)),
				_ => (),
			}

			line_number += 1;
		}

		return errors;
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn line_size() {
		let line_size = LineSize::new(5);

		assert_eq!(line_size.verify("", "1234\n").len(), 0);
		assert_eq!(line_size.verify("", "12345").len(), 1);
		assert_eq!(line_size.verify("", "12345\n").len(), 1);
		assert_eq!(line_size.verify("", "123456").len(), 1);
		assert_eq!(line_size.verify("", "12345\n1234\n1234").len(), 1);
		assert_eq!(line_size.verify("", "123456\n1234\n12345").len(), 2);
	}

	#[test]
	fn space_indentation() {
		let space_indentation = SpaceIndentation::new();

		assert_eq!(space_indentation.verify("", "             ").len(), 0);
		assert_eq!(space_indentation.verify("", " \n \n       ").len(), 0);
		assert_eq!(space_indentation.verify("", "1234\t56   \t").len(), 1);
		assert_eq!(space_indentation.verify("", "12345\t\t\n12345").len(), 1);
		assert_eq!(space_indentation.verify("", "12345\t\n1\t345\n12345").len(), 2);
		assert_eq!(space_indentation.verify("", "123456\n12\\t45\n123456").len(), 0);
	}

	#[test]
	fn trailing_whitespace() {
		let trailing_whitespace = TrailingWhiteSpace::new();
		
		assert_eq!(trailing_whitespace.verify("", "").len(), 0);
		assert_eq!(trailing_whitespace.verify("", "   a\n   b\n   o\n").len(), 0);
		assert_eq!(trailing_whitespace.verify("", "    12  34 \t56 ").len(), 1);
		assert_eq!(trailing_whitespace.verify("", "  \n \n").len(), 2);
		assert_eq!(trailing_whitespace.verify("", " z\t\t").len(), 1);
		assert_eq!(trailing_whitespace.verify("", " aad\t\n  \n \t").len(), 3);
	}
}
