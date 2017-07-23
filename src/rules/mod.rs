pub mod space;

pub use self::space::LineSize;

pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Option<Vec<String>>;
}
