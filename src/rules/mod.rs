pub mod space;
pub mod brace;
pub mod comment;
pub mod preprocessor;

pub use self::space::LineSize;
pub use self::space::SpaceIndentation;
pub use self::space::TrailingWhiteSpace;

pub use self::brace::OwnLineBrace;
pub use self::brace::IndentationLevel;

pub use self::comment::MultiLinesComment;

pub use self::preprocessor::PreprocessorOnFirstColumn;
pub use self::preprocessor::PreprocessorComment;

pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
