use rules::Rule;

pub struct OwnLineBrace {

}

impl OwnLineBrace {
	pub fn new() -> OwnLineBrace {
		OwnLineBrace {  }
	}
}

impl Rule for OwnLineBrace {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;		

		for line in content.lines() {
			if line.contains("{") && !(line.trim().len() == 1 ||
				(line.chars().filter(|c| !c.is_whitespace()).count() == 2 && line.trim_right().ends_with("\\"))) {
				errors.push(format!("[{}:{}]Opening brace must be on their own line.", filename, line_number));
			}
			else if line.contains("}") && !(line.trim().len() == 1 ||
					(line.chars().filter(|c| !c.is_whitespace()).count() == 2 && line.trim_right().ends_with("\\"))) {

				if !line.ends_with(";") {
					errors.push(format!("[{}:{}]Closing brace must be on their own line.", filename, line_number));
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct IndentationLevel {
	nb_spaces: usize
}

impl IndentationLevel {
	pub fn new(nb_spaces_per_indentation_level: usize) -> IndentationLevel {
		IndentationLevel { nb_spaces: nb_spaces_per_indentation_level }
	}
}

impl Rule for IndentationLevel {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;
		let mut indentation_level: usize = 0;

		for line in content.lines() {
			if line.contains("}") {
				if indentation_level > 0 {
					indentation_level -= 1;
				}
				else {
					errors.push(format!("[{}:{}]Unexpected closing brace.", filename, line_number));
				}
			}

			//Test of indentation
			if !line.trim().is_empty() {
				let current_indentation = line.len() - line.trim_left().len();
				if current_indentation != indentation_level * self.nb_spaces {
					errors.push(format!("[{}:{}]Wrong indentation level. Expected {} whitespaces got {}",
						filename, line_number, indentation_level * self.nb_spaces, current_indentation));
				} 
			}

			if line.contains("{") {
				indentation_level += 1;
			}

			line_number += 1;
		}

		if indentation_level > 0 {
			errors.push(format!("[{}:{}]Expected {} more closing brace.", filename, line_number, indentation_level));
		}

		return errors;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn own_line_brace() {
		let own_line_brace = OwnLineBrace::new();

		assert_eq!(own_line_brace.verify("", "{\nsome text\n}\n").len(), 0);
		assert_eq!(own_line_brace.verify("", "  {  ").len(), 0);
		assert_eq!(own_line_brace.verify("", "};").len(), 0);
		assert_eq!(own_line_brace.verify("", "} something;").len(), 0);//End of the typedef struct of do while loop.

		assert_ne!(own_line_brace.verify("", "{}\n").len(), 0);
		assert_ne!(own_line_brace.verify("", "}}").len(), 0);
		assert_ne!(own_line_brace.verify("", "{{").len(), 0);

		assert_eq!(own_line_brace.verify("", "{some").len(), 1);
		assert_eq!(own_line_brace.verify("", ";}").len(), 1);
		

	}

	#[test]
	fn indentation_level() {
		let indentation_level = IndentationLevel::new(4);

		assert_eq!(indentation_level.verify("", "             ").len(), 0);
		assert_eq!(indentation_level.verify("", "{\n    \n}   ").len(), 0);
		assert_eq!(indentation_level.verify("", "{\n    {\n    }\n}   ").len(), 0);

		//Missing closing and opening brace
		assert_eq!(indentation_level.verify("", "{").len(), 1);
		assert_eq!(indentation_level.verify("", "}").len(), 1);

		//Wrong indentation
		assert_eq!(indentation_level.verify("", "{\n 56\n}").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n     56\n}").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n    {\n  }\n}   ").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n    {\n    test\n    }\n}   ").len(), 1);
	}
}
