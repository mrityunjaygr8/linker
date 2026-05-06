use nanoid::nanoid;

pub fn generate() -> String {
    nanoid!(10)
    // "woo-123".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_length() {
        let id = generate();
        assert_eq!(10, id.len())
    }
}
