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
	rules.push(Box::new(rules::Comma::new()));

	//brace.rs
	rules.push(Box::new(rules::OwnLineBrace::new()));
	rules.push(Box::new(rules::IndentationLevel::new(4)));

	//misc.rs
	rules.push(Box::new(rules::MultiLinesComment::new()));
	rules.push(Box::new(rules::Goto::new()));
	rules.push(Box::new(rules::Enum::new()));
	rules.push(Box::new(rules::Semicolon::new()));
	rules.push(Box::new(rules::StaticVariable::new()));

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

	//function.rs
	rules.push(Box::new(rules::FunctionMaxCodeLines::new(25)));
	rules.push(Box::new(rules::FunctionMaxArguments::new(4)));
	rules.push(Box::new(rules::FunctionBlankLines::new()));
	rules.push(Box::new(rules::FunctionStartParenthesis::new()));
	rules.push(Box::new(rules::MaxFunctionsPerSourceFile::new(10)));
	rules.push(Box::new(rules::MaxExportedFunctions::new(5)));
	rules.push(Box::new(rules::FunctionParametersIndentation::new()));
	rules.push(Box::new(rules::FunctionsPrototypeLocation::new()));

	let mut files: Vec<String> = Vec::new();
	for arg in env::args().skip(1) {
		add_file_or_directory(&mut files, &arg);
	}

	verify(&rules, &files);
}

fn add_file_or_directory(mut files: &mut Vec<String>, pathname: &str)
{
	let path = Path::new(pathname);
	if path.is_file() {
		files.push(String::from(pathname));
	}
	else if path.is_dir() {
		for entry in path.read_dir().expect("something went wrong opening a directory.") {
    		if let Ok(entry) = entry {
        		add_file_or_directory(&mut files, entry.path().to_str().unwrap());
    		}
		}
	}
}

fn verify(rules: &Vec<Box<Rule>>, files: &Vec<String>)
{
	for filename in files {
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
