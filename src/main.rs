use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod rules;
use rules::Rule;

fn main() {
	let mut rules = Vec::<Box<Rule>>::new();
	//indentation.rs
	rules.push(Box::new(rules::LineSize::new(80)));
	rules.push(Box::new(rules::SpaceIndentation::new()));
	rules.push(Box::new(rules::TrailingWhiteSpace::new()));
	rules.push(Box::new(rules::IndentationLevel::new(4)));
	rules.push(Box::new(rules::Comma::new()));
	rules.push(Box::new(rules::StructureFieldsIndentation::new()));

	//naming.rs
	rules.push(Box::new(rules::Typedef::new()));
	rules.push(Box::new(rules::Global::new()));

	//misc.rs
	rules.push(Box::new(rules::OwnLineBrace::new()));
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

	//control_structures.rs
	rules.push(Box::new(rules::ControlStructuresIndentation::new()));
	rules.push(Box::new(rules::SpecialControlStructuresIndentation::new()));
	rules.push(Box::new(rules::SwitchDefaultCase::new()));
	rules.push(Box::new(rules::SwitchEnum::new()));
	rules.push(Box::new(rules::SwitchEnd::new()));	

	let mut filenames: Vec<String> = Vec::new();
	for arg in env::args().skip(1) {
		add_file_or_directory(&mut filenames, &arg);
	}

	verify(&rules, &filenames);
}

fn add_file_or_directory(mut filenames: &mut Vec<String>, pathname: &str) {
	let path = Path::new(pathname);
	if path.is_file() {
		filenames.push(String::from(pathname));
	}
	else if path.is_dir() {
		for entry in path.read_dir().expect(&format!("Something went wrong opening {}", pathname)) {
    		if let Ok(entry) = entry {
        		add_file_or_directory(&mut filenames, entry.path().to_str().unwrap());
    		}
		}
	}
}

fn verify(rules: &Vec<Box<Rule>>, filenames: &[String]) {
	for filename in filenames {
		let mut file = File::open(&filename).expect("file not found");
		let mut content = String::new();
		file.read_to_string(&mut content).expect(&format!("Something went wrong reading {}", filename));

		for rule in rules.iter() {
			for error in rule.verify(&filename, &content).iter() {
				println!("{}", error);
			}
		}
	}
}
