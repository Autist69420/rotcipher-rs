pub mod macros;

pub fn rot(text: &str, amount: u8) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + amount) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - amount) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rot13;

    #[test]
    fn does_rot13_work() {
        let coded = rot13!("hello");
        assert_eq!("uryyb", coded);
    }
}
