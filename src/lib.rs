extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate ansi_term;
pub fn from_lib() -> String {
    "hello, world".to_string()
}

pub mod data;
pub mod systems;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock() {
        assert_eq!("hello, world", from_lib());
    }
}
