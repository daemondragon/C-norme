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
			if line.trim_left().starts_with("static ") && (!line.contains("(") || line.contains("=")) && !line.contains("static const") {
				errors.push(format!("[{}:{}]Static variable must be const.", filename, line_number));
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
		assert_eq!(static_variable.verify("", "int some_static_name;").len(), 0);
		assert_eq!(static_variable.verify("", "//something static something").len(), 0);

		assert_eq!(static_variable.verify("", "static something;").len(), 1);
		assert_eq!(static_variable.verify("", "static var = function(parameter);").len(), 1);
	}
}
