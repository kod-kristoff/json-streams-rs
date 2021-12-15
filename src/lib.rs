pub mod json_iter;
pub mod json_lines;
mod json_value;
mod parsers;

// pub use json_lines::JsonLinesReader;

pub use json_value::JsonValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
