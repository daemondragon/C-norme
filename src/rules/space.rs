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
		None
	}
}
