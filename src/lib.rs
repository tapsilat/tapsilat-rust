/// Returns a greeting message
pub fn hello() -> String {
    "hello".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "hello");
    }
}
