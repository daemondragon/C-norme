pub mod space;
pub mod brace;
pub mod misc;
pub mod preprocessor;
pub mod function;

pub use self::space::*;
pub use self::brace::*;
pub use self::misc::*;
pub use self::preprocessor::*;
pub use self::function::*;


pub trait Rule {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
