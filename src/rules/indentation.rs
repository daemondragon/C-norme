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



pub struct Semicolon {
	
}

impl Semicolon {
	pub fn new() -> Semicolon {
		Semicolon { }
	}
}

impl Rule for Semicolon {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			match line.chars().filter(|x| *x == ';').count() {
				n if n == 1 => {
					if line.chars().last().unwrap() != ';' {
						errors.push(format!("[{}:{}]Semicolon must be followed by a newline.", filename, line_number));
					}
					else {
						let left_part = line.split(";").next().unwrap();
						if !left_part.trim_right().is_empty() &&
							left_part.trim_right().len() != left_part.len() {
							errors.push(format!("[{}:{}]Semicolon must not be precedeed by whitespaces.", filename, line_number));
						}
					}
				},
				n if n >= 2 => {
					if !line.contains("for") || n > 2 {
						errors.push(format!("[{}:{}]Too much semicolon found on this line.", filename, line_number));
					}
				},
				_ => {}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct Comma {
	
}

impl Comma {
	pub fn new() -> Comma {
		Comma { }
	}
}

impl Rule for Comma {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			let nb_comma = line.chars().filter(|x| *x == ',').count();
			let mut actual_between = 0;

			for between in line.split(",") {
				if actual_between < nb_comma {
					if actual_between != 0 && between.trim_left().len() + 1 != between.len() {
						errors.push(format!("[{}:{}]Comma must be followed by exactly one whitespace.", filename, line_number));
					}
					if between.trim_right().len() != between.len() {
						errors.push(format!("[{}:{}]Comma not must be precedeed by whitespaces.", filename, line_number))
					}
				}
				else if between.trim().is_empty() && !between.is_empty() {
					errors.push(format!("[{}:{}]The last comma must be followed by a newline.", filename, line_number));
				}

				actual_between += 1;
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

	#[test]
	fn semicolon() {
		let semicolon = Semicolon::new();

		assert_eq!(semicolon.verify("", ";").len(), 0);
		assert_eq!(semicolon.verify("", "something;").len(), 0);
		assert_eq!(semicolon.verify("", "	;").len(), 0);
		assert_eq!(semicolon.verify("", "for ( ; ;)").len(), 0);

		assert_ne!(semicolon.verify("", "for (;;);").len(), 0);
		assert_ne!(semicolon.verify("", "return ;").len(), 0);
		assert_ne!(semicolon.verify("", ";;;").len(), 0);
		assert_ne!(semicolon.verify("", ";\t").len(), 0);
	}

	#[test]
	fn comma() {
		let comma = Comma::new();

		assert_eq!(comma.verify("", "something,\nother").len(), 0);
		assert_eq!(comma.verify("", "something, something else\nother").len(), 0);
		assert_eq!(comma.verify("", "comma, comma, comma").len(), 0);

		assert_ne!(comma.verify("", "something, \nother").len(), 0);
		assert_ne!(comma.verify("", "something,\t\nother").len(), 0);
		assert_ne!(comma.verify("", "     , comma").len(), 0);
		assert_ne!(comma.verify("", "comma ,comma ,comma").len(), 0);
		assert_ne!(comma.verify("", "comma , comma , comma").len(), 0);
		assert_ne!(comma.verify("", "comma,comma,comma").len(), 0);
	}
}
