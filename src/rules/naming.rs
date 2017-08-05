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
			else if !in_macro && !line.trim_left().starts_with("//") && line.to_lowercase() != line {
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
}
