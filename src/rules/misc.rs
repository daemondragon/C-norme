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
				if in_enum && line.to_uppercase() != line {
					errors.push(format!("[{}:{}]Enum values must be entirely capitalized. Expected '{}' got '{}'", filename, line_number, line.to_uppercase(), line));
				}
				if line.contains(",") &&
					line.trim().len() > 1 &&//To prevent having only a comma on a line.
					!line.split(",").last().unwrap().trim().is_empty() {
					errors.push(format!("[{}:{}]Enum values must be on their own line.", filename, line_number));
				}
			}

			//Start_with '**' -> multilines comments intermediary lines.
			if line.contains("enum") && !line.trim_left().starts_with("//") && !line.trim_left().starts_with("**") {
				in_enum = true;
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct StaticVariable {
	
}

impl StaticVariable {
	pub fn new() -> StaticVariable {
		StaticVariable { }
	}
}

impl Rule for StaticVariable {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.contains("static") && (!line.contains("(") || line.contains("=")) && !line.contains("static const") {
				errors.push(format!("[{}:{}]Static variable must be const.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}



enum TypedefType {
	Normal,
	Struct,
	Enum,
	Union
}


pub struct Typedef {

}

impl Typedef {
	pub fn new() -> Typedef {
		Typedef { }
	}
}

impl Rule for Typedef {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut indentation = 0;
		let mut in_typedef = false;
		let mut typedef_type = TypedefType::Normal;

		for line in content.lines() {
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}

			if indentation <= 0 {
				if line.contains("typedef") {
					typedef_type = TypedefType::Normal;
					if line.contains("struct") {
						typedef_type = TypedefType::Struct;
					}
					else if line.contains("enum") {
						typedef_type = TypedefType::Enum;
					}
					else if line.contains("union") {
						typedef_type = TypedefType::Union;
					}

					in_typedef = true;
				}
				if in_typedef && line.contains(";") {
					let alias = line.split_whitespace().last().unwrap();
					if alias.len() <= 2 {
						errors.push(format!("[{}:{}]Anonymous typedef mustn't be used.", filename, line_number));
					}
					else if line.contains("typedef") && line.split_whitespace().count() == 3 {
						let old = line.split_whitespace().nth(1).unwrap();
						let new = line.split_whitespace().nth(2).unwrap();

						if ["s_", "u_", "e_", "t_", "f_"].iter().any(|x| old.starts_with(x)) {
							let start: String = old.chars().take(2).collect();
							if !new.starts_with(&start) {
								errors.push(format!("[{}:{}]Typedef '{}' must start with '{}'.", filename, line_number, alias, start));
							}
						}
					}
					else {
						match typedef_type {
							TypedefType::Normal if !(alias.starts_with("t_") || alias.starts_with("f_")) => {
								errors.push(format!("[{}:{}]Typedef '{}' must start with 't_' or 'f_'.", filename, line_number, alias));
							},
							TypedefType::Struct if !alias.starts_with("s_") => {
								errors.push(format!("[{}:{}]Struct typedef '{}' must start with 's_'.", filename, line_number, alias));
							},
							TypedefType::Enum if !alias.starts_with("e_") => {
								errors.push(format!("[{}:{}]Enum typedef '{}' must start with 'e_'.", filename, line_number, alias));
							},
							TypedefType::Union if !alias.starts_with("u_") => {
								errors.push(format!("[{}:{}]Union typedef '{}' must start with 'u_'.", filename, line_number, alias));
							},
							_ => {}
						}
					}
					in_typedef = false;
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
	fn static_variable() {
		let static_variable = StaticVariable::new();

		assert_eq!(static_variable.verify("", "something;").len(), 0);
		assert_eq!(static_variable.verify("", "const something;").len(), 0);
		assert_eq!(static_variable.verify("", "static const something;").len(), 0);
		assert_eq!(static_variable.verify("", "static function(parameter...").len(), 0);

		assert_eq!(static_variable.verify("", "static something;").len(), 1);
		assert_eq!(static_variable.verify("", "static var = function(parameter);").len(), 1);
	}

	#[test]
	fn typedef() {
		let typedef = Typedef::new();

		assert_eq!(typedef.verify("", "typedef unsigned char t_character;").len(), 0);
		assert_eq!(typedef.verify("", "typedef struct\n{\n}\n s_truct;").len(), 0);
		assert_eq!(typedef.verify("", "typedef enum\n{\n}\n e_num;").len(), 0);
		assert_eq!(typedef.verify("", "typedef union\n{\n}\n u_nion;").len(), 0);

		assert_eq!(typedef.verify("", "typedef unsigned char character;").len(), 1);
		assert_eq!(typedef.verify("", "typedef struct\n{\n}\n t_truct;").len(), 1);
		assert_eq!(typedef.verify("", "typedef enum\n{\n}\n s_num;").len(), 1);
		assert_eq!(typedef.verify("", "typedef union\n{\n}\n union;").len(), 1);


		assert_eq!(typedef.verify("", "typedef u_nion u_newunion;").len(), 0);
		assert_eq!(typedef.verify("", "typedef s_nion u_newunion;").len(), 1);
	}
}
