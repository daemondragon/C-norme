use std::env;
use std::fs::File;
use std::io::Read;

mod rules;
use rules::Rule;

fn main() {
	//RULES
	let mut rules = Vec::<Box<Rule>>::new();
	rules.push(Box::new(rules::LineSize::new(80)));
	rules.push(Box::new(rules::SpaceIndentation::new()));
	rules.push(Box::new(rules::TrailingWhiteSpace::new()));

	rules.push(Box::new(rules::OwnLineBrace::new()));
	rules.push(Box::new(rules::IndentationLevel::new(4)));

	rules.push(Box::new(rules::MultiLinesComment::new()));

	rules.push(Box::new(rules::PreprocessorOnFirstColumn::new()));
	rules.push(Box::new(rules::PreprocessorComment::new()));
	rules.push(Box::new(rules::MultiLinesMacro::new()));
	rules.push(Box::new(rules::MacroName::new()));

	//COMMAND LINE
	for filename in env::args().skip(1) {

		let mut file = File::open(&filename).expect("file not found");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("something went wrong reading the file");

		for rule in rules.iter() {
			for error in rule.verify(&filename, &content).iter() {
				println!("{}", error);
			}
		}
	}
}
