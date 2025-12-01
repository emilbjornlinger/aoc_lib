// Test function
pub fn info() -> String {
    String::from("This is an Advent of Code library written in rust")
}

// Expose grid module
pub mod grid;

// Expose combination module
pub mod combination;

// Expose input module
pub mod input;

// Expose modulo_wrap module
pub mod modulo_wrap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_test() {
        let result = info();
        assert!(result.contains("Advent of Code"));
    }
}
