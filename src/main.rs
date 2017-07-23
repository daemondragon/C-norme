mod rules;

use rules::Rule;

fn main() {
	let line_size = rules::LineSize::new(80);

	if let Some(errors) = line_size.verify("test.src", "Content") {
		for error in errors.iter() {
			println!("{}", error);
		}
	}
}
