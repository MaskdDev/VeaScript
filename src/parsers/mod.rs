// Import parser modules
pub mod embed;
pub mod math;
pub mod random;
pub mod string;
pub mod text;

// Re-export parsers
pub use embed::parse_embed as embed;
pub use math::parse_math as math;
pub use random::parse_random as random;
pub use string::parse_string as string;
pub use text::parse_text as text;
