pub fn from_lib() -> String {
    "hello, world".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock() {
        assert_eq!("hello, world", from_lib());
    }
}
