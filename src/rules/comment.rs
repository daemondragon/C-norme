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
}
