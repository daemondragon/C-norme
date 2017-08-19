pub mod indentation;
pub mod brace;
pub mod misc;
pub mod preprocessor;
pub mod function;
pub mod naming;
pub mod control_structures;

pub use self::indentation::*;
pub use self::brace::*;
pub use self::misc::*;
pub use self::preprocessor::*;
pub use self::function::*;
pub use self::naming::*;
pub use self::control_structures::*;


pub trait Rule: Sync {
	fn verify(&self, filename: &str, content: &str) -> Vec<String>;
}
