// Import parser modules
pub mod boolean;
pub mod decimal;
pub mod embed;
pub mod hexadecimal;
pub mod math;
pub mod random;
pub mod string;
pub mod text;

// Re-export parsers
pub use boolean::parse_bool as boolean;
pub use decimal::{int32, int64};
pub use embed::parse_embed as embed;
pub use hexadecimal::hexadecimal as hex;
pub use math::parse_math as math;
pub use random::parse_random as random;
pub use string::parse_string as string;
pub use text::parse_text as text;
