use rules::Rule;


pub struct FunctionMaxCodeLines {
	max_lines: usize
}

impl FunctionMaxCodeLines {
	pub fn new(nb_max_lines: usize) -> FunctionMaxCodeLines {
		FunctionMaxCodeLines { max_lines: nb_max_lines }
	}
}

impl Rule for FunctionMaxCodeLines {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut indentation = 0;
		let mut nb_code_lines = 0;

		for line in content.lines() {
			if line.contains("}") {
				indentation -= 1;
				if indentation == 0 && nb_code_lines > self.max_lines {
					errors.push(format!("[{}:{}]Function body's line count excedeed. Expected at most {} got {}", filename, line_number, self.max_lines, nb_code_lines));
				}
			}

			if indentation >= 1 {
				if !line.trim().is_empty() && !line.trim_left().starts_with("//") {
					nb_code_lines += 1;
				}
			}

			if line.contains("{") {
				indentation += 1;
				if indentation <= 1 {
					nb_code_lines = 0;
				}
			}

			line_number += 1;
		}


		return errors;
	}
}



pub struct FunctionMaxArguments {
	max_nb_arguments: usize
}

impl FunctionMaxArguments {
	pub fn new(nb_max_arguments: usize) -> FunctionMaxArguments {
		FunctionMaxArguments { max_nb_arguments: nb_max_arguments }
	}
}

impl Rule for FunctionMaxArguments {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut indentation = 0;
		let mut in_arguments = false;
		let mut nb_arguments = 0;

		for line in content.lines() {
			indentation += line.chars().filter(|x| *x == '{').count();
			indentation -= line.chars().filter(|x| *x == '}').count();

			let mut line = line;
			if indentation == 0 && line.contains("(") {
				line = line.split("(").nth(1).unwrap();
				in_arguments = true;
				nb_arguments = 0;
			}

			if in_arguments {
				line = line.split(")").next().unwrap();

				nb_arguments += line.split(",").count();

				if line.contains(")") {
					in_arguments = false;
					if nb_arguments > self.max_nb_arguments {
						errors.push(format!("[{}:{}]Too many function arguments. Expected at most {} got {}", filename, line_number, self.max_nb_arguments, nb_arguments));
					}
				}
			}

			line_number += 1;
		}


		return errors;
	}
}
