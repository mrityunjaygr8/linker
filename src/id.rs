use nanoid::nanoid;

pub fn generate() -> String {
    nanoid!(10)
    // "woo-123".to_string()
}
