mod rules;
use rules::Rule;

fn verify(rules: Vec<Box<Rule>>, filename: &str, content: &str) {
	for rule in rules.iter() {
		if let Some(errors) = rule.verify(filename, content) {
			for error in errors.iter() {
				println!("{}", error);
			}
		}
	}
}

fn main() {
	let mut rules = Vec::<Box<Rule>>::new();
	rules.push(Box::new(rules::LineSize::new(80)));

	verify(rules, "test.c", "content");	
}
