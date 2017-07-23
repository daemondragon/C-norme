use rules::Rule;

pub struct LineSize {
	max: usize
}

impl LineSize {
	pub fn new(max_line_size: usize) -> LineSize {
		LineSize { max: max_line_size }
	}
}

impl Rule for LineSize {
	fn verify(&self, filename: &str, content: &str) -> Option<Vec<String>> {
		let mut errors = Vec::new();
		let mut line_number: usize = 1;

		for line in content.lines() {
			if line.len() > self.max {
				errors.push(format!("[{}:{}]{}",filename, line_number, "Line size exceeded"));
			}

			line_number += 1;
		}

		match errors.len() {
			0 => None,
			_ => Some(errors),
		}
	}
}
