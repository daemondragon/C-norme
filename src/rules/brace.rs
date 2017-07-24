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
			if line.contains("{") && line.chars().filter(|c| !c.is_whitespace() && *c != '{').count() > 0 {
				errors.push(format!("[{}:{}]Opening brace must be on their own line.", filename, line_number));
			}
			else if line.contains("}") && line.chars().filter(|c| !c.is_whitespace() && *c != '}' && *c != ';').count() > 0 {
				errors.push(format!("[{}:{}]Closing brace must be on their own line.", filename, line_number));
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
			if line.trim().len() > 0 {
				let current_indentation = line.len() - line.trim_left().len();
				if current_indentation != indentation_level * self.nb_spaces {
					errors.push(format!("[{}:{}]Wrong indentation level. Expected {} got {}",
						filename, line_number, indentation_level * self.nb_spaces, current_indentation));
				} 
			}

			if line.contains("{") {
				indentation_level += 1;
			}

			line_number += 1;
		}

		if indentation_level > 0 {
			errors.push(format!("[{}:{}]Expected closing brace.", filename, line_number));
		}

		return errors;
	}
}
