pub fn parse_floatn(
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
    use crate::parse_floatn;

    #[test]
    fn parse_float() {
        let string = "1.124 34.6345 21.12".to_string();

        let mut white_space = string[..].split_whitespace();

        let mut result = Vec::new();

        parse_floatn(&mut white_space, &mut result, 3);

        assert_eq!(result.len(), 3);
        assert!(result.pop().eq(&Some(21.12)));
        assert!(result.pop().eq(&Some(34.6345)));
        assert!(result.pop().eq(&Some(1.124)));
    }
}
