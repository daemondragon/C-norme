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
		let mut in_multi_line_comment = false;

		for line in content.lines() {
			if line.contains("}") {
				indentation -= 1;
				if indentation == 0 && nb_code_lines > self.max_lines {
					errors.push(format!("[{}:{}]Function body's line count excedeed. Expected at most {} got {}", filename, line_number, self.max_lines, nb_code_lines));
				}
			}

			if indentation >= 1 {
				let mut dont_have_code = line.trim().is_empty() || line.trim_left().starts_with("//") || in_multi_line_comment; 
				if line.contains("/*") {
					in_multi_line_comment = true;
					dont_have_code = line.trim_left().starts_with("/*");
				}

				if line.contains("*/") && dont_have_code {
					in_multi_line_comment = false;
					dont_have_code = line.trim_right().ends_with("*/");
				}

				if !dont_have_code {
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
				if line.contains(")") {
					in_arguments = false;

					line = line.split(")").next().unwrap();
					nb_arguments += line.chars().filter(|c| *c == ',').count() + 1;// n ',' lead to n+1 arguments
					if nb_arguments > self.max_nb_arguments {
						errors.push(format!("[{}:{}]Too many function arguments. Expected at most {} got {}", filename, line_number, self.max_nb_arguments, nb_arguments));
					}
				}
				else {
					nb_arguments += line.chars().filter(|c| *c == ',').count();
				}
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
	fn function_max_code_lines() {
		let function_max_code_lines = FunctionMaxCodeLines::new(1);

		assert_eq!(function_max_code_lines.verify("", "something()\n{\n}").len(), 0);
		assert_eq!(function_max_code_lines.verify("", "something()\n{\n\n\n}").len(), 0);
		assert_eq!(function_max_code_lines.verify("", "something()\n{\ncode;\n\n}").len(), 0);
		assert_eq!(function_max_code_lines.verify("", "something()\n{\ncode;\n//comment\n}").len(), 0);
		assert_eq!(function_max_code_lines.verify("", "something()\n{\ncode;\n/*comment*/\n}").len(), 0);
		assert_eq!(function_max_code_lines.verify("", "something()\n{\ncode;\n/*\ncomment\n*/\n}").len(), 0);

		assert_ne!(function_max_code_lines.verify("", "something()\n{\ncode;\ncode;\n}").len(), 0);
		assert_ne!(function_max_code_lines.verify("", "something()\n{\ncode;\n//comment\ncode;\n}").len(), 0);
		assert_ne!(function_max_code_lines.verify("", "something()\n{\ncode;\ncode;//comment\n}").len(), 0);
		assert_ne!(function_max_code_lines.verify("", "something()\n{\ncode;\ncode/*\ncomment*/\n}").len(), 0);
		assert_ne!(function_max_code_lines.verify("", "something()\n{\ncode;\n/*\ncomment*/code;\n}").len(), 0);
	}

	#[test]
	fn function_max_arguments() {
		let function_max_arguments = FunctionMaxArguments::new(1);

		assert_eq!(function_max_arguments.verify("", "something()\n{\n}").len(), 0);
		assert_eq!(function_max_arguments.verify("", "something(int test)\n{\n}").len(), 0);
		assert_eq!(function_max_arguments.verify("", "something(struct test temp)\n{\n}").len(), 0);
		
		assert_eq!(function_max_arguments.verify("", "something(struct test temp, int testv2)\n{\n}").len(), 1);
		assert_eq!(function_max_arguments.verify("", "something(struct test temp, int testv2, int v4)\n{\n}").len(), 1);
		assert_eq!(function_max_arguments.verify("", "something(struct test temp,\nint testv2)\n{\n}").len(), 1);
		assert_eq!(function_max_arguments.verify("", "something(\nstruct test temp,\nint testv2\n)\n{\n}").len(), 1);
	}
}
