use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod rules;
use rules::Rule;

fn main() {
	let mut rules = Vec::<Box<Rule>>::new();
	//space.rs
	rules.push(Box::new(rules::LineSize::new(80)));
	rules.push(Box::new(rules::SpaceIndentation::new()));
	rules.push(Box::new(rules::TrailingWhiteSpace::new()));

	//brace.rs
	rules.push(Box::new(rules::OwnLineBrace::new()));
	rules.push(Box::new(rules::IndentationLevel::new(4)));

	//comment.rs
	rules.push(Box::new(rules::MultiLinesComment::new()));

	//preprocessor.rs
	rules.push(Box::new(rules::PreprocessorOnFirstColumn::new()));
	rules.push(Box::new(rules::PreprocessorIndentation::new()));
	rules.push(Box::new(rules::PreprocessorComment::new()));
	rules.push(Box::new(rules::MultiLinesMacro::new()));
	rules.push(Box::new(rules::MacroName::new()));
	rules.push(Box::new(rules::MacroArguments::new()));
	rules.push(Box::new(rules::IncludePreprocessor::new()));
	rules.push(Box::new(rules::IncludeOrder::new()));
	rules.push(Box::new(rules::HeaderGuard::new()));

	for arg in env::args().skip(1) {
		verify_file_or_directory(&rules, &arg);
		
	}
}

fn verify_file_or_directory(rules: &Vec<Box<Rule>>, pathname: &str)
{
	let path = Path::new(pathname);
	if path.is_file() {
		verify_file(&rules, pathname);
	}
	else if path.is_dir() {
		for entry in path.read_dir().expect("something went wrong opening a directory.") {
    		if let Ok(entry) = entry {
        		verify_file_or_directory(&rules, entry.path().to_str().unwrap());
    		}
		}
	} 
}

fn verify_file(rules: &Vec<Box<Rule>>, filename: &str)
{
	let mut file = File::open(&filename).expect("file not found");
	let mut content = String::new();
	file.read_to_string(&mut content).expect("something went wrong reading the file");

	for rule in rules.iter() {
		for error in rule.verify(&filename, &content).iter() {
			println!("{}", error);
		}
	}
}
