pub fn parse_float_n(
    val_str: &mut core::str::SplitWhitespace,
    val: &mut Vec<f32>,
    n: usize,
) -> bool {
    let sz = val.len();
    for string_repr in val_str.take(n) {
        match core::str::FromStr::from_str(string_repr) {
            Ok(mtl_param) => val.push(mtl_param),
            Err(_) => return false,
        }
    }
    sz + n == val.len()
}


#[cfg(test)]
mod util_test {
    use crate::parse_float_n;

    #[test]
    fn parse_float() {
        let string = "1.124 34.6345 21.12".to_string();

        let mut white_space = string[..].split_whitespace();

        let mut result = Vec::new();

        parse_float_n(&mut white_space, &mut result, 3);

        assert_eq!(result.len(), 3);
        assert!(result.pop().eq(&Some(21.12)));
        assert!(result.pop().eq(&Some(34.6345)));
        assert!(result.pop().eq(&Some(1.124)));
    }

    #[test]
    fn parse_invalid() {
        let string = "Rotation 180 20 130".to_string();
        let mut invalid_white_space = string[..].split_whitespace();

        let mut result = Vec::new();

        let is_valid = parse_float_n(&mut invalid_white_space, &mut result, 3);

        assert!(!is_valid);
        assert!(result.len().eq(&0));
    }
}
