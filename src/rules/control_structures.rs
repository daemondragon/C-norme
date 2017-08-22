use rules::Rule;



pub struct ControlStructuresIndentation {
	
}

impl ControlStructuresIndentation {
	pub fn new() -> ControlStructuresIndentation {
		ControlStructuresIndentation { }
	}
}

impl Rule for ControlStructuresIndentation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			for element in ["else if", "if", "for", "while", "switch", "return", "sizeof"].iter() {
				if line.trim_left().starts_with(element) && line.contains("(") && !line.contains(&(String::from(*element) + " (")) {
					errors.push(format!("[{}:{}]{} must be followed by ' ('.", filename, line_number, element));
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct SpecialControlStructuresIndentation {
	
}

impl SpecialControlStructuresIndentation {
	pub fn new() -> SpecialControlStructuresIndentation {
		SpecialControlStructuresIndentation { }
	}
}

impl Rule for SpecialControlStructuresIndentation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			for element in ["return", "break", "continue"].iter() {
				if line.trim_left().starts_with(element) && !line.contains("(") && !line.contains(&(String::from(*element) + ";")) {
					errors.push(format!("[{}:{}]{} must be directly followed by ';'.", filename, line_number, element));
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct SwitchEnum {
	
}

impl SwitchEnum {
	pub fn new() -> SwitchEnum {
		SwitchEnum { }
	}
}

impl Rule for SwitchEnum {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			for element in ["case", "default"].iter() {
				if line.trim_left().starts_with(element) {
					let without_element = line.replace(element, "");
					if without_element.to_uppercase() != without_element || without_element.find(char::is_numeric).is_some() {
						errors.push(format!("[{}:{}]Enum must only be used on enums.", filename, line_number));
					}
				}
			}

			line_number += 1;
		}

		return errors;
	}
}




pub struct SwitchDefaultCase {
	
}

impl SwitchDefaultCase {
	pub fn new() -> SwitchDefaultCase {
		SwitchDefaultCase { }
	}
}

//Expect ControlStructuresIndentation rule to be true.
impl Rule for SwitchDefaultCase {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut in_switch = false;
		let mut switch_indentation = 0;
		let mut found_default_case = false;

		for line in content.lines() {
			if line.trim_left().starts_with("switch (") {
				in_switch = true;
				found_default_case = false;
				switch_indentation = 0;
			}
			if in_switch {
				if line.contains("{") {
					switch_indentation += 1;
				}
				if switch_indentation > 0 && line.contains("}") {
					switch_indentation -= 1;
					if switch_indentation == 0 {
						in_switch = false;
						if !found_default_case {
							errors.push(format!("[{}:{}]Missing default case for the switch statement.", filename, line_number));
						}
					}
				}
				if line.trim_left().starts_with("default") {
					found_default_case = true;
				}
			}

			line_number += 1;
		}

		return errors;
	}
}



pub struct SwitchEnd {
	
}

impl SwitchEnd {
	pub fn new() -> SwitchEnd {
		SwitchEnd { }
	}
}

impl Rule for SwitchEnd {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut in_switch = false;
		let mut switch_indentation = 0;
		let mut have_seen_case_end = false;

		for line in content.lines() {
			if line.trim_left().starts_with("switch (") {
				in_switch = true;
				have_seen_case_end = true;
				switch_indentation = 0;
			}
			if in_switch {
				if line.contains("{") {
					switch_indentation += 1;
				}
				if switch_indentation > 0 && line.contains("}") {
					switch_indentation -= 1;
					if switch_indentation == 0 {
						in_switch = false;
						if !have_seen_case_end {
							errors.push(format!("[{}:{}]Missing return or break statement for the previous case.", filename, line_number));
						}
					}
				}
				if ["break", "return"].iter().any(|x| line.trim_left().starts_with(x)) {
					have_seen_case_end = true;
				}
				if ["case", "default"].iter().any(|x| line.trim_left().starts_with(x)) {
					if !have_seen_case_end {
						errors.push(format!("[{}:{}]Missing return or break statement for the previous case.", filename, line_number));
					}
					have_seen_case_end = false;
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
	fn control_structures_indentation()
	{
		let control_structures_indentation = ControlStructuresIndentation::new();

		assert_eq!(control_structures_indentation.verify("", "something,\nother").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "if (condition)").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "while (condition)").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "switch (condition)").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "else if (condition)").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "for (i = 0; i < n; ++i)").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "return (something);").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "return;").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "sizeof (something);").len(), 0);

		assert_eq!(control_structures_indentation.verify("", "#if").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "rediffusion").len(), 0);

		assert_eq!(control_structures_indentation.verify("", "if(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "while(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "switch(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "else if(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "for(i = 0; i < n; ++i)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "return(something);").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "sizeof(something);").len(), 1);

		assert_eq!(control_structures_indentation.verify("", "if\t(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "while  (condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "return   (i);").len(), 1);

		assert_eq!(control_structures_indentation.verify("", "int name_while = f(arg);").len(), 0);

	}

	#[test]
	fn special_control_structures_indentation()
	{
		let special_control_structures_indentation = SpecialControlStructuresIndentation::new();

		assert_eq!(special_control_structures_indentation.verify("", "something,\nother").len(), 0);
		assert_eq!(special_control_structures_indentation.verify("", "return (condition);").len(), 0);
		assert_eq!(special_control_structures_indentation.verify("", "return;").len(), 0);
		assert_eq!(special_control_structures_indentation.verify("", "continue;").len(), 0);
		assert_eq!(special_control_structures_indentation.verify("", "break;").len(), 0);

		assert_eq!(special_control_structures_indentation.verify("", "return ;").len(), 1);
		assert_eq!(special_control_structures_indentation.verify("", "continue\n;").len(), 1);
		assert_eq!(special_control_structures_indentation.verify("", "break     ;").len(), 1);
		assert_eq!(special_control_structures_indentation.verify("", "return").len(), 1);
	}

	#[test]
	fn switch_enum()
	{
		let switch_enum = SwitchEnum::new();

		assert_eq!(switch_enum.verify("", "case ENUM_TYPE:").len(), 0);
		assert_eq!(switch_enum.verify("", "default:").len(), 0);

		assert_eq!(switch_enum.verify("", "case 1:").len(), 1);
		assert_eq!(switch_enum.verify("", "case something_else;").len(), 1);
	}

	#[test]
	fn switch_default_case() {
		let switch_default_case = SwitchDefaultCase::new();

		assert_eq!(switch_default_case.verify("", "switch (something)\n{\ndefault:\nf()\nbreak;\n}").len(), 0);
		assert_eq!(switch_default_case.verify("", "switch (something)\n{\ncase 1:\ng()\nbreak;\ndefault:\nf()\nbreak;\n}").len(), 0);
		assert_eq!(switch_default_case.verify("", "switch (something)\n{\ncase 1:\n{\ng()\nbreak;\n}\ndefault:\nf()\nbreak;\n}").len(), 0);

		assert_eq!(switch_default_case.verify("", "switch (something)\n{\n}").len(), 1);
		assert_eq!(switch_default_case.verify("", "switch (something)\n{\ncase 1:\nf()\nbreak;\n}").len(), 1);

		assert_eq!(switch_default_case.verify("", "void switch_something(bool b)\n{\n}").len(), 0);
		assert_eq!(switch_default_case.verify("", "void\nswitch_something\n(bool b)\n{\n}").len(), 0);
	}

	#[test]
	fn switch_end() {
		let switch_end = SwitchEnd::new();

		assert_eq!(switch_end.verify("", "switch (something)\n{\n}").len(), 0);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ndefault:\nf()\nbreak;\n}").len(), 0);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\nf()\nbreak;\n}").len(), 0);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\n{\ng()\nbreak;\n}\ncase:\nf()\nbreak;\n}").len(), 0);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\ng()\nbreak;\ndefault:\nf()\nbreak;\n}").len(), 0);
		
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\nf()\n}").len(), 1);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ndefault 1:\nf()\n}").len(), 1);

		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\ng();\ncase:\nf()\nbreak;\n}").len(), 1);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\ng();\ndefault:\nf()\nbreak;\n}").len(), 1);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\ng();\nbreak;\n\ncase:\nf();\n}").len(), 1);
		assert_eq!(switch_end.verify("", "switch (something)\n{\ncase 1:\ng();\nbreak;\ndefault:\nf();\n}").len(), 1);
	}
}
