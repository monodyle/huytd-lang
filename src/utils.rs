pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    let mut digits_end = 0;

    for (idx, c) in s.char_indices() {
        if c.is_ascii_digit() {
            digits_end = idx + 1;
        } else {
            break;
        }
    }

    let digits = &s[..digits_end];
    let remainder = &s[digits_end..];
    (remainder, digits)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }
}
