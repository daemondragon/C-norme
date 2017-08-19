use rules::Rule;


pub struct ControlStructuresIndentation {
	
}

impl ControlStructuresIndentation {
	pub fn new() -> ControlStructuresIndentation {
		ControlStructuresIndentation { }
	}
}

//Expect OwnLineBrace rule to be true.
impl Rule for ControlStructuresIndentation {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			for element in ["else if", "if", "for", "while", "switch"].iter() {
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

//Expect OwnLineBrace rule to be true.
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

		assert_eq!(control_structures_indentation.verify("", "#if").len(), 0);
		assert_eq!(control_structures_indentation.verify("", "rediffusion").len(), 0);

		assert_eq!(control_structures_indentation.verify("", "if(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "while(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "switch(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "else if(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "for(i = 0; i < n; ++i)").len(), 1);

		assert_eq!(control_structures_indentation.verify("", "if\t(condition)").len(), 1);
		assert_eq!(control_structures_indentation.verify("", "while  (condition)").len(), 1);

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
}
