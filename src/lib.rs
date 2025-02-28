pub fn info() -> String {
    String::from("This is an Advent of Code library written in rust")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_test() {
        let result = info();
        assert!(result.contains("Advent of Code"));
    }
}
