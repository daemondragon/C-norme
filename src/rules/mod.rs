pub mod space;

pub use self::space::LineSize;
pub use self::space::SpaceIndentation;
pub use self::space::TrailingWhiteSpace;

pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
