use rules::Rule;

pub struct LowercaseNames {

}

impl LowercaseNames {
	pub fn new() -> LowercaseNames {
		LowercaseNames { }
	}
}

impl Rule for LowercaseNames {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		if filename.to_lowercase() != filename {
			errors.push(format!("[{}]File name must be in lowercase.", filename));
		}

		let mut in_macro = false;

		for line in content.lines() {
			if line.starts_with("#") {
				in_macro = true;
			}
			else if !in_macro && !line.trim_left().starts_with("//") && !line.trim_left().starts_with("**") && line.replace("NULL", "null").to_lowercase() != line {
				errors.push(format!("[{}:{}]Functions and variables name must be in lowercase.", filename, line_number));
			}

			if in_macro {
				in_macro = line.ends_with("\\");
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
	fn lowercase_names() {
		let lowercase_names = LowercaseNames::new();

		assert_eq!(lowercase_names.verify("test.h", "zdnkcndccc").len(), 0);
		assert_eq!(lowercase_names.verify("", "#define MACRO").len(), 0);
		assert_eq!(lowercase_names.verify("", "#define\\MACRO").len(), 0);
		assert_eq!(lowercase_names.verify("", "f(int arg1, int arg2)\n{\n}").len(), 0);

		assert_ne!(lowercase_names.verify("Test.c", "dccc").len(), 0);
		assert_ne!(lowercase_names.verify("", "f(int ARG1, int arg2)\n{\n}").len(), 0);
		assert_ne!(lowercase_names.verify("", "Function(int arg1, int arg2)\n{\n}").len(), 0);
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
