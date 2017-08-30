use rules::Rule;

use std::str::Lines;
use std::iter::Peekable;

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


#[derive(Debug, PartialEq)]
enum IndentationType {
	Basic,//indentation made by {}
	OneLine,//'if', 'else' ... that can have only the next line indented.
	SwitchCase
}


pub struct IndentationLevel {
	nb_spaces: usize
}

impl IndentationLevel {
	pub fn new(nb_spaces_per_indentation_level: usize) -> IndentationLevel {
		IndentationLevel { nb_spaces: nb_spaces_per_indentation_level }
	}

	fn verify_for_level(&self, filename: &str, lines: &mut Peekable<Lines>, indentation_type: IndentationType, indentation_level: usize,
		errors: &mut Vec<String>, line_number: &mut usize)
	{
		if indentation_type == IndentationType::OneLine {
			*line_number += 1;
			if let Some(line) = lines.next() {
				let current_indentation = line.len() - line.trim_left().len();
				if current_indentation != indentation_level * self.nb_spaces {
					errors.push(format!("[{}:{}]Wrong indentation level. Expected {} whitespaces got {}",
						filename, line_number, indentation_level * self.nb_spaces, current_indentation));
				}
			}
		}
		else {
			while let Some(line) = lines.next() {
				*line_number += 1;

				let current_indentation = line.len() - line.trim_left().len();
				let needed_indentation = match indentation_type {
					IndentationType::Basic if line.trim_left().starts_with('}') => (indentation_level - 1) * self.nb_spaces,
					_ => indentation_level * self.nb_spaces,
				};

				if line.len() != 0 && current_indentation != needed_indentation {
					errors.push(format!("[{}:{}]Wrong indentation level. Expected {} whitespaces got {}",
						filename, line_number, needed_indentation, current_indentation));
				}

				let mut is_one_line = false;
				if ["if", "else", "switch", "while", "for"].iter().any(|x| line.trim_left().starts_with(x)) {
					if let Some(next_line) = lines.peek() {
						if !next_line.trim_left().starts_with('{') {
							is_one_line = true;
						}
					}
				}
				if is_one_line {
					self.verify_for_level(filename, lines, IndentationType::OneLine, indentation_level + 1, errors, line_number);
				}

				if indentation_type == IndentationType::Basic {
					if line.trim_left().starts_with('}') {
						break;
					}
					else if line.trim_left().starts_with('{') {
						self.verify_for_level(filename, lines, IndentationType::Basic, indentation_level + 1, errors, line_number);
					}
					else if ["case", "default"].iter().any(|x| line.trim_left().starts_with(x)) {
						self.verify_for_level(filename, lines, IndentationType::SwitchCase, indentation_level + 1, errors, line_number);
					}
				}
				else if indentation_type == IndentationType::SwitchCase {
					if line.trim_left().starts_with("break") {
						break;
					}
					else if  line.trim_left().starts_with("return") {
						if let Some(next_line) = lines.peek() {
							if !next_line.trim_left().starts_with("break") {
								break;
							}
						}
					}
				}
				else {
					unreachable!();
				}
			}
		}
	}
}

impl Rule for IndentationLevel {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 0;

		let mut lines = content.lines().peekable();

		self.verify_for_level(filename, &mut lines, IndentationType::Basic, 0, &mut errors, &mut line_number);

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

//Expect IndentationLevel rule to be true
impl Rule for Semicolon {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			match line.chars().filter(|x| *x == ';').count() {
				n if n == 1 => {
					if line.contains(" while") {
						errors.push(format!("[{}:{}]Too much semicolon found on this line.", filename, line_number));
					}

					if !line.ends_with(";") && !line.ends_with("\\"){
						errors.push(format!("[{}:{}]Semicolon must be followed by a newline.", filename, line_number));
					}
					
					let left_part = line.split(";").next().unwrap();
					if !left_part.trim_right().is_empty() &&
						left_part.trim_right().len() != left_part.len() {
						errors.push(format!("[{}:{}]Semicolon must not be precedeed by whitespaces.", filename, line_number));
					}
				},
				n if n >= 2 => {
					if !line.contains(" for") || n > 2 {
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

			if nb_comma <= 0 {
				continue;
			}

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



pub struct StructureFieldsIndentation {
	
}

impl StructureFieldsIndentation {
	pub fn new() -> StructureFieldsIndentation {
		StructureFieldsIndentation { }
	}
}

//Expect Semicolon, OwnLineBrace and Trailing WhiteSpace rules to be true.
impl Rule for StructureFieldsIndentation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut in_structure = false;
		let mut have_typedef = false;
		let mut level = 0;
		let mut indentation = 0;


		for line in content.lines() {
			if line.contains("{") {
				level += 1;
			}
			if line.contains("}") && level > 0 {
				level -= 1;
			}

			if level <= 0 && (line.contains("struct") || line.contains("union")) &&
				!(line.contains("(") || line.contains(",") || line.contains(")") ||
					line.trim_left().starts_with("//") || line.trim_left().starts_with("**")) {
				//Too avoid been triggered in function declaration.
				in_structure = true;
				have_typedef = line.contains("typedef");
			}

			if in_structure {
				if !line.contains("{") && line.trim().len() > 0 && !(!have_typedef && line.contains("}")) && (!line.contains("typedef") || line.split_whitespace().count() > 2) {
					//Indentation check is needed (else can be '{' or '};')
					let current_indentation = line.len() - line.split_whitespace().last().unwrap().len();
					if indentation <= 0 {
						indentation = current_indentation;
					}
					else if current_indentation != indentation
					{
						errors.push(format!("[{}:{}]Wrong field indentation. Expected {} got {}.", filename, line_number, indentation, current_indentation));
					}
				}
				if line.contains("}") {
					in_structure = false;
					indentation = 0;
				}
			}

			line_number += 1;
		}

		return errors;
	}
}


#[cfg(test)]
mod tests {
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
	fn indentation_level() {
		let indentation_level = IndentationLevel::new(4);

		assert_eq!(indentation_level.verify("", "{\n    \n}   ").len(), 0);
		assert_eq!(indentation_level.verify("", "{\n    {\n    }\n}   ").len(), 0);
		assert_eq!(indentation_level.verify("", "if (true)\n    f();").len(), 0);
		assert_eq!(indentation_level.verify("", "switch (number)\n{\n    case 1:\n        f();\n        break;\n    case 2:\n        return g();\n}").len(), 0);

		//Wrong indentation
		assert_eq!(indentation_level.verify("", "             ").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n 56\n}").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n     56\n}").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n    {\n  }\n}   ").len(), 1);
		assert_eq!(indentation_level.verify("", "{\n    {\n    test\n    }\n}   ").len(), 1);

		assert_eq!(indentation_level.verify("", "if (true)\nf();").len(), 1);
		assert_eq!(indentation_level.verify("", "if (true)\n    f();\n    g();").len(), 1);

		assert_eq!(indentation_level.verify("", "switch (number)\n{\ncase 1:\n    f();\n    break;\ncase 2:\n    g();\n    break;\n}").len(), 6);
		assert_eq!(indentation_level.verify("", "switch (number)\n{\n    case 1:\n    f();\n    break;\n    case 2:\n    g();\n    break;\n}").len(), 4);
	}

	#[test]
	fn semicolon() {
		let semicolon = Semicolon::new();

		assert_eq!(semicolon.verify("", ";").len(), 0);
		assert_eq!(semicolon.verify("", "something;").len(), 0);
		assert_eq!(semicolon.verify("", "	;").len(), 0);
		assert_eq!(semicolon.verify("", "    for ( ; ;)").len(), 0);
		assert_eq!(semicolon.verify("", "#define MACRO(something) \\\n do_something();   \\\n other_things();").len(), 0);

		assert_ne!(semicolon.verify("", " for (;;);").len(), 0);
		assert_ne!(semicolon.verify("", " return ;").len(), 0);
		assert_ne!(semicolon.verify("", ";;;").len(), 0);
		assert_ne!(semicolon.verify("", ";\t").len(), 0);
	}

	#[test]
	fn comma() {
		let comma = Comma::new();

		assert_eq!(comma.verify("", "something,\nother").len(), 0);
		assert_eq!(comma.verify("", "something, something else\nother").len(), 0);
		assert_eq!(comma.verify("", "comma, comma, comma").len(), 0);
		assert_eq!(comma.verify("", "{\n    {\n    \n}\n}").len(), 0);

		assert_ne!(comma.verify("", "something, \nother").len(), 0);
		assert_ne!(comma.verify("", "something,\t\nother").len(), 0);
		assert_ne!(comma.verify("", "     , comma").len(), 0);
		assert_ne!(comma.verify("", "comma ,comma ,comma").len(), 0);
		assert_ne!(comma.verify("", "comma , comma , comma").len(), 0);
		assert_ne!(comma.verify("", "comma,comma,comma").len(), 0);  
	}

	#[test]
	fn structure_fields_indentation()
	{
		let structure_fields_indentation = StructureFieldsIndentation::new();

		assert_eq!(structure_fields_indentation.verify("", "struct  test\n{\n    int arg1;\n};\n").len(), 0);
		assert_eq!(structure_fields_indentation.verify("", "struct  test\n{\n    int arg1;\n    int arg2;\n};\n").len(), 0);
		assert_eq!(structure_fields_indentation.verify("", "typedef struct test\n{\n           int arg1;\n}              s_test;\n").len(), 0);
		assert_eq!(structure_fields_indentation.verify("", "typedef struct\n{\n           int arg1;\n}              s_test;\n").len(), 0);


		assert_eq!(structure_fields_indentation.verify("", "struct   test\n{\n    int arg1;\n};\n").len(), 1);
		assert_eq!(structure_fields_indentation.verify("", "struct  test\n{\n   int arg1;\n     int arg2;\n};\n").len(), 2);
		assert_eq!(structure_fields_indentation.verify("", "typedef struct test\n{\n           int arg1;\n}     s_test;\n").len(), 1);
		assert_eq!(structure_fields_indentation.verify("", "typedef struct\n{\n    int arg1;\n} s_test;\n").len(), 1);
	}
}
