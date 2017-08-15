use rules::Rule;


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



pub struct Global {

}

impl Global {
	pub fn new() -> Global {
		Global { }
	}
}

impl Rule for Global {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut indentation = 0;
		let mut in_macro = false;

		let mut global_variable_found = 0;

		for line in content.lines() {
			if line.contains("{") {
				indentation += 1;
			}
			if line.contains("}") && indentation > 0 {
				indentation -= 1;
			}

			if indentation <= 0 {
				if line.trim_left().starts_with("#") {
					in_macro = line.trim_right().ends_with("\\");
				}
				else if !in_macro {
					if line.contains(";") && !line.contains("}") && (!line.contains("(") || line.contains("=")) &&
						(!line.trim_left().starts_with("//") && !line.trim_left().starts_with("**")){
						global_variable_found += 1;
						let macro_name = line.split("=").next().unwrap();
						let macro_name = macro_name.split_whitespace().last().unwrap();

						if !macro_name.starts_with("g_") {
							errors.push(format!("[{}:{}]Macro name '{}' must start with 'g_'.", filename, line_number, macro_name));
						}
					}
				}
				else {
					in_macro = line.trim_right().ends_with("\\");
				}
			}

			line_number += 1;
		}

		if global_variable_found > 1 {
			errors.push(format!("[{}]One global variable per file maximum, found {}.", filename, global_variable_found));
		}

		return errors;
	}
}



#[cfg(test)]
mod tests {
	use super::*;
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

	#[test]
	fn global() {
		let global = Global::new();

		assert_eq!(global.verify("", "#define something;").len(), 0);
		assert_eq!(global.verify("", "int g_something;").len(), 0);
		assert_eq!(global.verify("", "int g_something = f();").len(), 0);
		assert_eq!(global.verify("", "void function();").len(), 0);

		assert_eq!(global.verify("", "int name;").len(), 1);
		assert_eq!(global.verify("", "struct something name;").len(), 1);

		assert_eq!(global.verify("", "int g_name = function();").len(), 0);
		assert_eq!(global.verify("", "//Comment;").len(), 0);
		assert_eq!(global.verify("", "/*\n**MultiLine Comment;\n*/").len(), 0);
	}
}
