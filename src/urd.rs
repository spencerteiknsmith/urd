use lazy_static;

use bimap::BiMap;

pub enum Mode {
    ToUrd,
    FromUrd,
}

lazy_static::lazy_static! {
    static ref URD: BiMap<char, char> = {
        let mut b = BiMap::new();
        let keeb: Vec<Vec<char>> = vec![
            "`1234567890-=".chars().collect(),
            "qwertyuiop[]\\".chars().collect(),
            "asdfghjkl;'".chars().collect(),
            "zxcvbnm,./".chars().collect(),
            "~!@#$%^&*()_+".chars().collect(),
            "QWERTYUIOP{}|".chars().collect(),
            "ASDFGHJKL:\"".chars().collect(),
            "ZXCVBNM<>?".chars().collect(),
        ];
        for row in keeb {
            for i in 0..row.len() - 1 {
                b.insert(row[i], row[i+1]);
            }
            b.insert(row[row.len() - 1], row[0]);
        }
        b.insert(' ', ' ');
        b
    };
}

fn urd_char(c: char) -> char {
    *URD.get_by_left(&c).unwrap_or(&'?')
}

fn deurd_char(c: char) -> char {
    *URD.get_by_right(&c).unwrap_or(&'?')
}

pub fn convert(s: &str, mode: &Mode) -> String {
    s.chars().map(match mode {
        Mode::ToUrd => urd_char,
        Mode::FromUrd => deurd_char,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_og_urd() {
        assert_eq!(convert("yes", &Mode::ToUrd), "urd");
    }

    #[test]
    fn reverse_og_urd() {
        assert_eq!(convert("urd", &Mode::FromUrd), "yes");
    }

    #[test]
    fn right_edge_urd() {
        assert_eq!(convert("=\\'/", &Mode::ToUrd), "`qaz");
    }

    #[test]
    fn right_edge_deurd() {
        assert_eq!(convert("=\\'/", &Mode::FromUrd), "-];.");
    }
}