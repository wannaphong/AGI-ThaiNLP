//! Thai character constants.

/// 44 Thai consonants (พยัญชนะไทย)
pub const THAI_CONSONANTS: &str = "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ";

/// 20 Thai vowels (สระไทย)
pub const THAI_VOWELS: &str = "\u{0e24}\u{0e26}\u{0e30}\u{0e31}\u{0e32}\u{0e33}\u{0e34}\u{0e35}\u{0e36}\u{0e37}\u{0e38}\u{0e39}\u{0e40}\u{0e41}\u{0e42}\u{0e43}\u{0e44}\u{0e45}\u{0e4d}\u{0e47}";

/// 5 Thai lead vowels (สระนำ): เ แ โ ใ ไ
pub const THAI_LEAD_VOWELS: &str = "\u{0e40}\u{0e41}\u{0e42}\u{0e43}\u{0e44}";

/// 4 Thai follow vowels (สระตาม): ะ า ำ ๅ
pub const THAI_FOLLOW_VOWELS: &str = "\u{0e30}\u{0e32}\u{0e33}\u{0e45}";

/// 7 Thai above vowels (สระบน): ั ิ ี ึ ื ็ ็
pub const THAI_ABOVE_VOWELS: &str = "\u{0e31}\u{0e34}\u{0e35}\u{0e36}\u{0e37}\u{0e4d}\u{0e47}";

/// 2 Thai below vowels (สระล่าง): ุ ู
pub const THAI_BELOW_VOWELS: &str = "\u{0e38}\u{0e39}";

/// 4 Thai tonemarks (วรรณยุกต์): ่ ้ ๊ ๋
pub const THAI_TONEMARKS: &str = "\u{0e48}\u{0e49}\u{0e4a}\u{0e4b}";

/// 6 Thai signs that can be part of a word: ฯ ฺ ๆ ์ ็ ๎
pub const THAI_SIGNS: &str = "\u{0e2f}\u{0e3a}\u{0e46}\u{0e4c}\u{0e4d}\u{0e4e}";

/// 3 Thai section markers: ๏ ๚ ๛
pub const THAI_PUNCTUATIONS: &str = "\u{0e4f}\u{0e5a}\u{0e5b}";

/// 10 Thai digits: ๐-๙
pub const THAI_DIGITS: &str = "๐๑๒๓๔๕๖๗๘๙";

/// Thai Baht symbol ฿
pub const THAI_SYMBOLS: &str = "\u{0e3f}";

/// Check whether a Unicode scalar value is in the Thai block (U+0E00–U+0E7F).
#[inline]
pub fn is_thai_char(c: char) -> bool {
    ('\u{0e00}'..='\u{0e7f}').contains(&c)
}

/// Check whether a string contains any Thai character.
pub fn contains_thai(s: &str) -> bool {
    s.chars().any(is_thai_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consonants_count() {
        assert_eq!(THAI_CONSONANTS.chars().count(), 44);
    }

    #[test]
    fn vowels_count() {
        assert_eq!(THAI_VOWELS.chars().count(), 20);
    }

    #[test]
    fn lead_vowels_count() {
        assert_eq!(THAI_LEAD_VOWELS.chars().count(), 5);
    }

    #[test]
    fn tonemarks_count() {
        assert_eq!(THAI_TONEMARKS.chars().count(), 4);
    }

    #[test]
    fn digits_count() {
        assert_eq!(THAI_DIGITS.chars().count(), 10);
    }

    #[test]
    fn is_thai_char_test() {
        assert!(is_thai_char('ก'));
        assert!(is_thai_char('ฮ'));
        assert!(!is_thai_char('a'));
        assert!(!is_thai_char('A'));
    }

    #[test]
    fn contains_thai_test() {
        assert!(contains_thai("กาแฟ"));
        assert!(!contains_thai("hello"));
        assert!(contains_thai("hello กา"));
    }
}
