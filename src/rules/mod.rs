pub mod space;
pub mod brace;
pub mod comment;
pub mod preprocessor;

pub use self::space::{LineSize,
					  SpaceIndentation,
					  TrailingWhiteSpace};
pub use self::brace::{OwnLineBrace,
					  IndentationLevel};
pub use self::comment::{MultiLinesComment};
pub use self::preprocessor::{PreprocessorOnFirstColumn,
							 PreprocessorIndentation,
							 PreprocessorComment,
							 MultiLinesMacro,
							 MacroName,
							 IncludePreprocessor,
							 IncludeOrder,
							 HeaderGuard};


pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
