pub mod space;
pub mod brace;
pub mod comment;

pub use self::space::LineSize;
pub use self::space::SpaceIndentation;
pub use self::space::TrailingWhiteSpace;

pub use self::brace::OwnLineBrace;
pub use self::brace::IndentationLevel;

pub use self::comment::MultiLinesComment;

pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
