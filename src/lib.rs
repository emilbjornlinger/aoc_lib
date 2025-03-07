// Test function
pub fn info() -> String {
    String::from("This is an Advent of Code library written in rust")
}

// Expose grid module
mod grid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_test() {
        let result = info();
        assert!(result.contains("Advent of Code"));
    }
}
