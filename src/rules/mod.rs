pub mod space;
pub mod brace;

pub use self::space::LineSize;
pub use self::space::SpaceIndentation;
pub use self::space::TrailingWhiteSpace;

pub use self::brace::OwnLineBrace;
pub use self::brace::IndentationLevel;

pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
