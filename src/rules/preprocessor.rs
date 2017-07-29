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
			if line.contains("#") && !line.starts_with("#") {
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

impl Rule for PreprocessorComment {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.contains("#") && 
				["endif", "else"].iter().any(|x| line.contains(x)) &&
				!(line.contains("/*") && line.contains("*/")) && !line.contains("//")  {
					errors.push(format!("[{}:{}]Else and endif directives must have a comment describing their initial condition.", filename, line_number));
			}

			line_number += 1;
		}

		return errors;
	}
}

pub struct MultiLinesPreprocessor {
	
}

impl MultiLinesPreprocessor {
	pub fn new() -> MultiLinesPreprocessor {
		MultiLinesPreprocessor {  }
	}
}

impl Rule for MultiLinesPreprocessor {
	fn verify(&self, filename: &str, content: &str) -> Vec<String> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		let mut in_multi_line_macro = false;
		let mut alignement = 0;

		for line in content.lines() {
			if line.contains("#") {
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


#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn preprocessor_on_first_column() {
		let preprocessor_on_first_column = PreprocessorOnFirstColumn::new();

		assert_eq!(preprocessor_on_first_column.verify("", "#\n#").len(), 0);

		assert_eq!(preprocessor_on_first_column.verify("", " #\n\t#").len(), 2);
		assert_eq!(preprocessor_on_first_column.verify("", "3#something").len(), 1);
		assert_eq!(preprocessor_on_first_column.verify("", "adee#").len(), 1);
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
	fn multi_lines_preprocessor() {
		let multi_lines_preprocessor = MultiLinesPreprocessor::new();

		assert_eq!(multi_lines_preprocessor.verify("", "qdee\ncece").len(), 0);
		assert_eq!(multi_lines_preprocessor.verify("", "#define efes 10").len(), 0);
		assert_eq!(multi_lines_preprocessor.verify("", "#define zefrg (azdd ad) \\\n czdeff\n").len(), 0);

		assert_eq!(multi_lines_preprocessor.verify("", "#define  zefrg (azdd ad) \\\n czdeff\\\n").len(), 1);
		assert_eq!(multi_lines_preprocessor.verify("", "#define  zefrg (azdd ad) \\\n czdeff                  \\\neececev").len(), 0);
	}
}
