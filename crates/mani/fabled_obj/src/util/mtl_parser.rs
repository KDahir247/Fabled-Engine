pub fn parse_floatn(
    val_str: &mut core::str::SplitWhitespace,
    val: &mut Vec<f32>,
    n: usize,
) -> bool {
    let sz = val.len();
    for b in val_str.take(n) {
        match core::str::FromStr::from_str(b) {
            Ok(a) => val.push(a),
            Err(_) => return false,
        }
    }
    sz + n == val.len()
}
