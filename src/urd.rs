//! # URD
//!
//! This module translates text to and from "urd" text.
//! "urd" text is normal text as if it was typed with hands transposed one position on the keyboard.
//!
//! # Edges of the keyboard
//!
//! To deal with the edges of the keyboard, characters are wrapped around the keyboard.
//! It is as if the keyboard were cylindrical.

use lazy_static;

use bimap::BiMap;

/// Specifies direction of translation
///
/// Effectively a semantic boolean
pub enum Mode {
    ToUrd,
    FromUrd,
}

lazy_static::lazy_static! {
    // Build a bidirectional translation map for each character.
    //
    // Rather than specify the map manually, this code makes it easier
    // to specify what they keyboard looks like and build the map from it.
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

/// Translate a single character one keyboard position to the right
fn urd_char(c: char) -> char {
    *URD.get_by_left(&c).unwrap_or(&'?')
}

/// Translate a single character one keyboard position to the left
fn deurd_char(c: char) -> char {
    *URD.get_by_right(&c).unwrap_or(&'?')
}

/// Convert a string `s` according to the provided `mode`.
///
/// # Examples
///
/// ```
/// let yes = "yes";
/// let urd = urd::convert(yes, &urd::Mode::ToUrd);
/// assert_eq!(urd, "urd");
/// ```
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