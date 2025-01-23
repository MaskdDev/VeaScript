/// Convert a 32 bit integer to a 6 character (lowercase) hexadecimal string
pub fn hex_to_str(hexadecimal: i32) -> String {
    // Get base hexadecimal string
    let mut hex_string = format!("{:x}", hexadecimal);

    // Add 0s to the start if needed
    let zeroes_needed = 6 - hex_string.len();
    hex_string = "0".repeat(zeroes_needed) + &hex_string;

    // Return hex string
    return hex_string;
}
