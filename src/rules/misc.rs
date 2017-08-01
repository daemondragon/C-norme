use rules::Rule;

pub struct MultiLinesComment {
}

impl MultiLinesComment {
	pub fn new() -> MultiLinesComment {
		MultiLinesComment { }
	}
}

impl Rule for MultiLinesComment {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;
		
		let mut in_comment = false;

		for line in content.lines() {
			if line.contains("*/") && !line.contains("/*") {
				//Multi lines comment
				if !in_comment {
					errors.push(format!("[{}:{}]Unexpected comment end delimiter.", filename, line_number));
				}

				if line.chars().filter(|c| !c.is_whitespace()).count() != 2 {
					errors.push(format!("[{}:{}]Comment end delimiter must appear on its own line.", filename, line_number));
				}
				in_comment = false;
			}

			if in_comment {
				if !line.trim_left().starts_with("**") {
					errors.push(format!("[{}:{}]Comment intermediary line must start with '**'.", filename, line_number));
				}
			}

			if line.contains("/*") && !line.contains("*/") {
				//Multi lines comment
				if in_comment {
					errors.push(format!("[{}:{}]Comments can't be nested.", filename, line_number));
				}

				let nb_non_white_space = line.chars().filter(|c| !c.is_whitespace()).count();
				if !(nb_non_white_space == 2 || (nb_non_white_space == 3 && line.contains("/**"))) {
					errors.push(format!("[{}:{}]Comment start delimiter must appear on its own line.", filename, line_number));
				}
				in_comment = true;

			}

			line_number += 1;
		}

		if in_comment {
			errors.push(format!("[{}:{}]Expected comment end delimiter.", filename, line_number));
		}

		return errors;
	}
}



pub struct Goto {

}

impl Goto {
	pub fn new() -> Goto {
		Goto { }
	}
}

impl Rule for Goto {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.contains("goto") {
				errors.push(format!("[{}:{}]Goto statement unauthorized.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct Enum {
	
}

impl Enum {
	pub fn new() -> Enum {
		Enum { }
	}
}

impl Rule for Enum {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut in_enum = false;

		for line in content.lines() {
			if in_enum {
				if line.contains("}") {
					in_enum = false;
				}
				if line.to_uppercase() != line {
					errors.push(format!("[{}:{}]Enum values must be entirely capitalized.", filename, line_number));
				}
				if line.contains(",") &&
					line.trim().len() > 1 &&//To prevent having only a comma on a line.
					!line.split(",").last().unwrap().trim().is_empty() {
					errors.push(format!("[{}:{}]Enum values must be on their own line.", filename, line_number));
				}
			}

			if line.contains("enum") {
				in_enum = true;
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



#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn multi_lines_comment() {
		let multi_lines_comment = MultiLinesComment::new();

		assert_eq!(multi_lines_comment.verify("", "zdnkcndccc").len(), 0);
		assert_eq!(multi_lines_comment.verify("", "//zdnkcndccc").len(), 0);
		assert_eq!(multi_lines_comment.verify("", "/*zdnkcndccc*/").len(), 0);
		assert_eq!(multi_lines_comment.verify("", "/*\n**zdnkcn\n*/").len(), 0);

		assert_ne!(multi_lines_comment.verify("", "/*zdnkcn\ndccc*/").len(), 0);
		assert_ne!(multi_lines_comment.verify("", "/*\nzdnkcn\n*/").len(), 0);
		assert_ne!(multi_lines_comment.verify("", "/*\nav**zdnkcn\n*/").len(), 0);
		assert_ne!(multi_lines_comment.verify("", "/** *\n**zdnkcn\n*/").len(), 0);
		assert_ne!(multi_lines_comment.verify("", "/**\n**zdnkcn\n*/*").len(), 0);
	}

	#[test]
	fn goto() {
		let goto = Goto::new();

		assert_eq!(goto.verify("", "zdnkcndccc").len(), 0);
		assert_eq!(goto.verify("", "go\nto\ngo\nto\n").len(), 0);
		assert_eq!(goto.verify("", "goto").len(), 1);
		assert_eq!(goto.verify("", "goto\nadezf\nvvrgotoded").len(), 2);
	}

	#[test]
	fn enum_rule() {
		let enum_rule = Enum::new();

		assert_eq!(enum_rule.verify("", "enum{}").len(), 0);
		assert_eq!(enum_rule.verify("", "enum\n{\n}A").len(), 0);

		assert_eq!(enum_rule.verify("", "enum\n{\nVALUE\n}").len(), 0);
		assert_eq!(enum_rule.verify("", "enum\n{\nVALUE, \t\nVALUE2\n}").len(), 0);

		assert_eq!(enum_rule.verify("", "enum\n{\nvalue\n}").len(), 1);
		assert_eq!(enum_rule.verify("", "enum\n{\nValue\n}").len(), 1);
		assert_eq!(enum_rule.verify("", "enum\n{\nVALUE,VALUE2\n}").len(), 1);
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
}
