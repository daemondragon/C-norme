use rules::Rule;

pub struct PreprocessorOnFirstColumn {

}

impl PreprocessorOnFirstColumn {
	pub fn new() -> PreprocessorOnFirstColumn {
		PreprocessorOnFirstColumn {  }
	}
}

impl Rule for PreprocessorOnFirstColumn {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.trim_left().starts_with("#") && !line.starts_with("#") {
				errors.push(format!("[{}:{}]Preprocessor directive must start on the first column.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct PreprocessorComment {
	
}

impl PreprocessorComment {
	pub fn new() -> PreprocessorComment {
		PreprocessorComment {  }
	}
}

//Expect PreprocessorOnFirstColumn rule true for the given file
impl Rule for PreprocessorComment {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.starts_with("#") && 
				["endif", "else"].iter().any(|x| line.contains(x)) &&
				!(line.contains("/*") && line.contains("*/")) && !line.contains("//")  {
					errors.push(format!("[{}:{}]Else and endif directives must have a comment describing their initial condition.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct MultiLinesMacro {
	
}

impl MultiLinesMacro {
	pub fn new() -> MultiLinesMacro {
		MultiLinesMacro {  }
	}
}

//Expect PreprocessorOnFirstColumn rule true for the given file
impl Rule for MultiLinesMacro {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut in_multi_line_macro = false;
		let mut alignement = 0;

		for line in content.lines() {
			if line.starts_with("#") {
				if line.trim_right().ends_with("\\") {
					in_multi_line_macro = true;
					alignement = line.rfind("\\").unwrap();
				}
			}

			if in_multi_line_macro {
				match line.rfind("\\") {
					Some(index) if index != alignement => errors.push(format!("[{}:{}]Multi lines macro must have \\ aligned. Expected alignement on column {} got {}", filename, line_number, alignement, index)),
					Some(_) => {},
					None => { in_multi_line_macro = false; },
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct MacroName {
	
}

impl MacroName {
	pub fn new() -> MacroName {
		MacroName {  }
	}
}

//Expect PreprocessorOnFirstColumn rule true for the given file,
//and the macro name being on the first line of the macro (even for multiline macro).
impl Rule for MacroName {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.starts_with("#") && line.contains("def") {
				//Match #ifndef, #ifdef and #define
				
				let words: Vec<&str> = line.split_whitespace().collect();
				let macro_name = if words[0].contains("def") { words.get(1) } else { words.get(2) };
				if let Some(macro_name) = macro_name {
					let macro_name = macro_name.split("(").next().unwrap();

					if macro_name != macro_name.to_uppercase() {
						errors.push(format!("[{}:{}]Macro must have be entirely capitalized. Expected \"{}\" got \"{}\"", filename, line_number, macro_name.to_uppercase(), macro_name));
					}
				}
			}

			line_number += 1;
		}

		return errors;
	}
}


//All #include directive must appear at the start of the file.
pub struct IncludePreprocessor {
	
}

impl IncludePreprocessor {
	pub fn new() -> IncludePreprocessor {
		IncludePreprocessor {  }
	}
}

//Expect PreprocessorOnFirstColumn rule true for the given file,
impl Rule for IncludePreprocessor {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut have_seen_code = false;
		let mut multiline_macro = false;

		for line in content.lines() {
			if line.starts_with("#") {
				if line.contains("\\") {
					multiline_macro = true;
				}
				if line.contains("include") && have_seen_code {
					errors.push(format!("[{}:{}]All #include directive must appear at the start of the file.", filename, line_number));
				}
			}
			else
			{
				if !multiline_macro && line.trim().len() > 0 {
					have_seen_code = true;
				}
				if multiline_macro && !line.contains("\\") {
					multiline_macro = false;
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



//System header must appear before local one. In header (.h) file only.
pub struct IncludeOrder {
	
}

impl IncludeOrder {
	pub fn new() -> IncludeOrder {
		IncludeOrder {  }
	}
}

//Expect PreprocessorOnFirstColumn rule true for the given file,
impl Rule for IncludeOrder {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		if !filename.contains(".h") {
			return Vec::new();//Not a header.
		}

		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut have_seen_local_include = false;

		for line in content.lines() {
			if line.starts_with("#") && line.contains("include") {
				if line.contains("\"") {
					have_seen_local_include = true;
				}
				else if line.contains("<") && have_seen_local_include
				{
					errors.push(format!("[{}:{}]System headers must appear before locals one.", filename, line_number));
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct HeaderGuard {
	
}

impl HeaderGuard {
	pub fn new() -> HeaderGuard {
		HeaderGuard {  }
	}
}

//Expect PreprocessorOnFirstColumn rule true for the given file
impl Rule for HeaderGuard {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		if !filename.contains(".h") {
			return Vec::new();//Not a header.
		}

		let mut errors = Vec::new();
		let header_guard = filename.split("/").last().unwrap().replace(".", "_").to_uppercase() + "_";

		let first_line = String::from("#ifndef ") + &header_guard;
		let second_line = String::from("# define ") + &header_guard;

		match content.lines().nth(0) {
			Some(line) if line == first_line => {},
			_ => {
				errors.push(format!("[{}:1]'{}' must appear on the first line.", filename, first_line));
			}
		}
		match content.lines().nth(1) {
			Some(line) if line == second_line => {},
			_ => {
				errors.push(format!("[{}:1]'{}' must appear on the second line.", filename, second_line));
			}
		}

		return errors;
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn preprocessor_on_first_column() {
		let preprocessor_on_first_column = PreprocessorOnFirstColumn::new();

		assert_eq!(preprocessor_on_first_column.verify("", "#\n#").len(), 0);

		assert_eq!(preprocessor_on_first_column.verify("", " #\n\t#").len(), 2);
		assert_eq!(preprocessor_on_first_column.verify("", "3#something").len(), 0);
		assert_eq!(preprocessor_on_first_column.verify("", "adee#").len(), 0);
	}

	#[test]
	fn preprocessor_comment() {
		let preprocessor_comment = PreprocessorComment::new();

		assert_eq!(preprocessor_comment.verify("", "qdee\ncece").len(), 0);
		assert_eq!(preprocessor_comment.verify("", "#if").len(), 0);
		assert_eq!(preprocessor_comment.verify("", "#define").len(), 0);

		assert_eq!(preprocessor_comment.verify("", "#else").len(), 1);
		assert_eq!(preprocessor_comment.verify("", "#endif").len(), 1);

		assert_eq!(preprocessor_comment.verify("", "#else /*  */").len(), 0);
		assert_eq!(preprocessor_comment.verify("", "#endif /* */").len(), 0);

		assert_eq!(preprocessor_comment.verify("", "#else //").len(), 0);
		assert_eq!(preprocessor_comment.verify("", "#endif //").len(), 0);
	}

	#[test]
	fn multi_lines_macro() {
		let multi_lines_macro = MultiLinesMacro::new();

		assert_eq!(multi_lines_macro.verify("", "qdee\ncece").len(), 0);
		assert_eq!(multi_lines_macro.verify("", "#define efes 10").len(), 0);
		assert_eq!(multi_lines_macro.verify("", "#define zefrg (azdd ad) \\\n czdeff\n").len(), 0);

		assert_eq!(multi_lines_macro.verify("", "#define  zefrg (azdd ad) \\\n czdeff\\\n").len(), 1);
		assert_eq!(multi_lines_macro.verify("", "#define  zefrg (azdd ad) \\\n czdeff                  \\\neececev").len(), 0);
	}

	#[test]
	fn macro_name() {
		let macro_name = MacroName::new();

		assert_eq!(macro_name.verify("", "#ifdef HELLO").len(), 0);
		assert_eq!(macro_name.verify("", "# define WORLD 10").len(), 0);
		assert_eq!(macro_name.verify("", "#   ifndef MACRO_NAME(lower_case)\\\n something\n").len(), 0);

		assert_eq!(macro_name.verify("", "#define  Name").len(), 1);
		assert_eq!(macro_name.verify("", "#define  name_lower_Case").len(), 1);
		assert_eq!(macro_name.verify("", "#define  zefrg(TEST ad)\\\n czdeff").len(), 1);
	}

	#[test]
	fn include_preprocessor() {
		let include_preprocessor = IncludePreprocessor::new();

		assert_eq!(include_preprocessor.verify("", "#ifdef HELLO").len(), 0);
		assert_eq!(include_preprocessor.verify("", "something").len(), 0);
		assert_eq!(include_preprocessor.verify("", "# include").len(), 0);

		assert_eq!(include_preprocessor.verify("", "#define something\n#include").len(), 0);
		assert_eq!(include_preprocessor.verify("", "#define something\\\nend_of_mcro_definition\n#include").len(), 0);

		assert_eq!(include_preprocessor.verify("", "code\n#  include \"no_code\"").len(), 1);
		assert_eq!(include_preprocessor.verify("", "#include name\n code \n#include other\n").len(), 1);
	}

	#[test]
	fn include_order() {
		let include_order = IncludeOrder::new();

		assert_eq!(include_order.verify(".h", "#ifdef HELLO").len(), 0);
		assert_eq!(include_order.verify(".h", "# include").len(), 0);
		assert_eq!(include_order.verify(".h", "#define SOMETHING\n# include \"header.h\"").len(), 0);
		assert_eq!(include_order.verify(".h", "#define SOMETHING\n# include <header.h>").len(), 0);
		assert_eq!(include_order.verify(".h", "# include <header.h>\n# include \"header.h\"").len(), 0);

		assert_eq!(include_order.verify("test.h", "# include \"header.h\"\n# include <header.h>\n").len(), 1);
		assert_eq!(include_order.verify("test.h", "# include <header.h>\n# include \"header.h\"\n# include <header.h>\n").len(), 1);

		assert_eq!(include_order.verify("hello.c", "#define SOMETHING\n# include <header.h>").len(), 0);
		assert_eq!(include_order.verify(".c", "# include <header.h>\n# include \"header.h\"").len(), 0);
		assert_eq!(include_order.verify(".c", "# include \"header.h\"\n# include <header.h>\n").len(), 0);
		assert_eq!(include_order.verify(".c", "# include <header.h>\n# include \"header.h\"\n# include <header.h>\n").len(), 0);
	}

	#[test]
	fn header_guard() {
		let header_guard = HeaderGuard::new();

		assert_eq!(header_guard.verify("test.h", "#ifndef TEST_H_\n# define TEST_H_").len(), 0);
		assert_eq!(header_guard.verify("../src/test.h", "#ifndef TEST_H_\n# define TEST_H_").len(), 0);

		assert_eq!(header_guard.verify("test.h", "#ifndef TEST_H\n# define TEST_H\n").len(), 2);
		assert_ne!(header_guard.verify("test.h", "# define TEST_H_").len(), 0);
		assert_ne!(header_guard.verify("test.h", "#ifndef TEST_H_").len(), 0);

		assert_eq!(header_guard.verify("test.h", "#ifndef TEST_H__\n# define TEST_H__").len(), 2);

		assert_ne!(header_guard.verify("test.h", "#ifndef TEST_H_#define TEST_H_").len(), 0);
		assert_eq!(header_guard.verify("test.h", "#ifndef TEST_H_\n\n# define TEST_H_").len(), 1);
		assert_eq!(header_guard.verify("test.h", "#ifndef TEST_H_\n#define OTHER_H\n# define TEST_H_").len(), 1);
	}
}
