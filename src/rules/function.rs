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
					errors.push(format!("[{}:{}]Function body's line count excedeed. Expected at most {} got {}.", filename, line_number, self.max_lines, nb_code_lines));
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
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}

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
						errors.push(format!("[{}:{}]Too many function arguments. Expected at most {} got {}.", filename, line_number, self.max_nb_arguments, nb_arguments));
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



pub struct FunctionBlankLines {
}

impl FunctionBlankLines {
	pub fn new() -> FunctionBlankLines {
		FunctionBlankLines { }
	}
}

impl Rule for FunctionBlankLines {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut indentation = 0;
		let mut is_previous_blank = false;
		let mut in_multi_line_comment = false;

		for line in content.lines() {
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}

			if indentation > 0 {
				if line.trim().is_empty() && !in_multi_line_comment {
					if is_previous_blank {
						errors.push(format!("[{}:{}]Two following blank line found.", filename, line_number));
					}
					is_previous_blank = true;
				}
				else
				{
					let mut dont_have_code = line.trim_left().starts_with("//") || in_multi_line_comment ; 
					if line.contains("/*") {
						in_multi_line_comment = true;
						dont_have_code = line.trim_left().starts_with("/*");
					}

					if line.contains("*/") && dont_have_code {
						in_multi_line_comment = false;
						dont_have_code = line.trim_right().ends_with("*/");
					}

					is_previous_blank = dont_have_code && is_previous_blank;
				}
			}

			line_number += 1;
		}


		return errors;
	}
}



pub struct FunctionStartParenthesis {

}

impl FunctionStartParenthesis {
	pub fn new() -> FunctionStartParenthesis {
		FunctionStartParenthesis { }
	}
}

impl Rule for FunctionStartParenthesis {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut indentation = 0;
		
		for line in content.lines() {
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}
			
			if indentation == 0 {
				match line.chars().position(|x| x == '(') {
					Some(p) if p > 0 => {
						if line.chars().nth(p - 1).unwrap().is_whitespace() {
							errors.push(format!("[{}:{}]Function parenthesis must be next to function name.", filename, line_number));
						}
					},
					_ => (),
				}
			}

			line_number += 1;
		}


		return errors;
	}
}



pub struct MaxFunctionsPerSourceFile {
	max_functions: usize
}

impl MaxFunctionsPerSourceFile {
	pub fn new(max_functions_per_source_file: usize) -> MaxFunctionsPerSourceFile {
		MaxFunctionsPerSourceFile { max_functions: max_functions_per_source_file }
	}
}

//This rule expect OwnLineBrace and IndentationLevel rule to be true
impl Rule for MaxFunctionsPerSourceFile {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		if !filename.contains(".c") {
			return Vec::new();
		}

		let mut errors = Vec::new();
		let mut nb_functions: usize = 0;
		
		for line in content.lines() {			
			if line.starts_with("{") {
				nb_functions += 1;
			}
		}

		if nb_functions > self.max_functions {
			errors.push(format!("[{}]Too much functions found. Expected at most {} functions, got {}.", filename, self.max_functions, nb_functions));
		}


		return errors;
	}
}



pub struct MaxExportedFunctions {
	max_functions: usize
}

impl MaxExportedFunctions {
	pub fn new(max_exported_functions: usize) -> MaxExportedFunctions {
		MaxExportedFunctions { max_functions: max_exported_functions }
	}
}

//This rule expect the opening parenthesis of macro to be on the first line of the macro
impl Rule for MaxExportedFunctions {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		if !filename.contains(".h") {
			return Vec::new();
		}

		let mut errors = Vec::new();
		let mut nb_functions: usize = 0;
		
		for line in content.lines() {			
			if line.contains("(") && !line.starts_with("#") && !line.ends_with("\\"){
				nb_functions += 1;
			}
		}

		if nb_functions > self.max_functions {
			errors.push(format!("[{}]Too much exported functions found. Expected at most {} functions, got {}.", filename, self.max_functions, nb_functions));
		}


		return errors;
	}
}



pub struct FunctionParametersIndentation {
}

impl FunctionParametersIndentation {
	pub fn new() -> FunctionParametersIndentation {
		FunctionParametersIndentation {  }
	}
}

//This rule expect OwnLineBrace rule to be true
impl Rule for FunctionParametersIndentation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;
		
		let mut indentation = 0;
		let mut in_function_parameters = false;
		let mut parameters_indentation = 0;

		for line in content.lines() {
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}

			if indentation <= 0 {
				if line.contains("(") {
					in_function_parameters = true;
					parameters_indentation = line.split("(").next().unwrap().len() + 1;
				}
				else if in_function_parameters {
					let current_indentation = line.len() - line.trim_left().len();
					if current_indentation != parameters_indentation {
						errors.push(format!("[{}:{}]Wrong parameters indentation level. Expected {} functions, got {}.", filename, line_number, parameters_indentation, current_indentation));
					}
				}
			}

			if in_function_parameters && line.contains(")") {
				in_function_parameters = false;
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct FunctionsPrototypeLocation {
}

impl FunctionsPrototypeLocation {
	pub fn new() -> FunctionsPrototypeLocation {
		FunctionsPrototypeLocation {  }
	}
}

// This rule expect OwnLineBrace rule to be true
// and macro functions end parenthesis to be on the first line of the macro.
impl Rule for FunctionsPrototypeLocation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		if !filename.contains(".c") {
			return Vec::new();
		}
		let mut errors = Vec::new();
		let mut line_number: usize = 1;
		
		let mut indentation = 0;

		for line in content.lines() {
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}

			if indentation <= 0 && line.contains(")") && line.contains(";"){
				errors.push(format!("[{}:{}]Functions prototype must be located in header file.", filename, line_number));
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

	#[test]
	fn function_blank_lines() {
		let function_blank_lines = FunctionBlankLines::new();

		assert_eq!(function_blank_lines.verify("", "f()\n{\n}").len(), 0);
		assert_eq!(function_blank_lines.verify("", "f()\n{\n\n}").len(), 0);
		assert_eq!(function_blank_lines.verify("", "f()\n{\n//comment\n//comment\n}").len(), 0);
		assert_eq!(function_blank_lines.verify("", "f()\n{\ncode\n\n}").len(), 0);
		assert_eq!(function_blank_lines.verify("", "f()\n{\n/*comment\n*/\n\n}").len(), 0);

		assert_eq!(function_blank_lines.verify("", "f()\n{\n\n\n}").len(), 1);
		assert_eq!(function_blank_lines.verify("", "f()\n{\n\n//comment\n\n}").len(), 1);
		assert_eq!(function_blank_lines.verify("", "f()\n{\n\n/*\ncomment\n*/\n\n}").len(), 1);
		assert_eq!(function_blank_lines.verify("", "f()\n{\n\n/*next line is a comment\n\n*/\n\n}").len(), 1);
	}

	#[test]
	fn function_start_parenthesis() {
		let function_start_parenthesis = FunctionStartParenthesis::new();

		assert_eq!(function_start_parenthesis.verify("", "f()\n{\n}").len(), 0);
		assert_eq!(function_start_parenthesis.verify("", "f()\n{\n (\n}").len(), 0);
		assert_eq!(function_start_parenthesis.verify("", "f()\n{\n\t(\n}").len(), 0);

		assert_eq!(function_start_parenthesis.verify("", "f ()\n{\n\n}").len(), 1);
		assert_eq!(function_start_parenthesis.verify("", "f   ()\n{\n\n}").len(), 1);
		assert_eq!(function_start_parenthesis.verify("", "f\t()\n{\n\n}").len(), 1);
	}

	#[test]
	fn max_functions_per_source_file() {
		let max_functions_per_source_file = MaxFunctionsPerSourceFile::new(1);

		assert_eq!(max_functions_per_source_file.verify(".c", "something();").len(), 0);
		assert_eq!(max_functions_per_source_file.verify(".c", "f()\n{\n\n}").len(), 0);
		assert_eq!(max_functions_per_source_file.verify(".c", "f ()\n{\n {\n }\n}").len(), 0);

		assert_eq!(max_functions_per_source_file.verify(".c", "{\n}\n{\n}").len(), 1);
		assert_eq!(max_functions_per_source_file.verify(".c", "{\n}\n{\n}\n{\n}").len(), 1);
		assert_eq!(max_functions_per_source_file.verify(".h", "f()\n{\n}\ng()\n{\n}").len(), 0);
		assert_eq!(max_functions_per_source_file.verify(".h", "f ()\n{\n {\n }\n}").len(), 0);
	}

	#[test]
	fn max_exported_functions() {
		let max_exported_functions = MaxExportedFunctions::new(1);

		assert_eq!(max_exported_functions.verify(".c", "something();\nsomething();").len(), 0);
		assert_eq!(max_exported_functions.verify(".h", "f();").len(), 0);

		assert_eq!(max_exported_functions.verify(".h", "f()\ng()\n").len(), 1);
		assert_eq!(max_exported_functions.verify(".h", "#m()\ng()").len(), 0);
	}

	#[test]
	fn function_parameters_indentation() {
		let function_parameters_indentation = FunctionParametersIndentation::new();

		assert_eq!(function_parameters_indentation.verify("", "something();\nsomething();").len(), 0);
		assert_eq!(function_parameters_indentation.verify("", "something(hello);\nsomething(world);").len(), 0);
		assert_eq!(function_parameters_indentation.verify("", "something(hello, world);").len(), 0);
		assert_eq!(function_parameters_indentation.verify("", "f(hello,\n  world);").len(), 0);
		assert_eq!(function_parameters_indentation.verify("", "f(hello,\n  world\n  and);").len(), 0);

		assert_eq!(function_parameters_indentation.verify("", "f(hello,\n world);").len(), 1);
		assert_eq!(function_parameters_indentation.verify("", "f(hello,\n   world);").len(), 1);
		
		assert_eq!(function_parameters_indentation.verify("", "f(hello,\n   world\nand);").len(), 2);
	}

	#[test]
	fn functions_prototype_location() {
		let functions_prototype_location = FunctionsPrototypeLocation::new();

		assert_eq!(functions_prototype_location.verify(".h", "something();\nsomething();").len(), 0);

		assert_eq!(functions_prototype_location.verify(".c", "something(hello);\nsomething(world);").len(), 2);
		assert_eq!(functions_prototype_location.verify(".c", "something(hello)\n{\n}\nsomething(world){}").len(), 0);
	}
}
