use std::env;
use std::fs::File;
use std::io::Read;

mod rules;
use rules::Rule;

fn verify(rules: &Vec<Box<Rule>>, filename: &str, content: &str) {
	for rule in rules.iter() {
		if let Some(errors) = rule.verify(filename, content) {
			for error in errors.iter() {
				println!("{}", error);
			}
		}
	}
}

fn main() {
	//RULES
	let mut rules = Vec::<Box<Rule>>::new();
	rules.push(Box::new(rules::LineSize::new(80)));

	//COMMAND LINE
	for arg in env::args().skip(1) {
		
		let mut file = File::open(&arg).expect("file not found");
    	let mut content = String::new();
    	file.read_to_string(&mut content).expect("something went wrong reading the file");

		verify(&rules, &arg, &content);	
	}
}
