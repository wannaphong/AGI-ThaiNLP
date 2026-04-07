//! Utility functions: Thai collation, digit conversion, number words, normalization.

use once_cell::sync::Lazy;
use regex::Regex;

// ---------------------------------------------------------------------------
// Tonemarks and lead-vowel normalization for collation
// ---------------------------------------------------------------------------

static RE_TONE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\u{0e48}\u{0e49}\u{0e4a}\u{0e4b}\u{0e47}\u{0e4c}]").unwrap()
});
static RE_LEAD_VOWEL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([\u{0e40}-\u{0e44}])([\u{0e01}-\u{0e2e}])").unwrap()
});

fn thai_sort_key(word: &str) -> String {
    // Swap lead vowel + consonant → consonant + lead vowel
    let key = RE_LEAD_VOWEL.replace_all(word, "$2$1").to_string();
    // Remove tonemarks
    RE_TONE.replace_all(&key, "").to_string()
}

/// Sort strings (almost) according to Thai dictionary order.
///
/// Tonemarks and lead vowels (เ แ โ ใ ไ) are normalized before comparison.
///
/// # Examples
/// ```
/// use agi_thainlp::collate;
/// let mut words = vec!["ไก่", "เกิด", "กาล"];
/// let sorted = collate(&words, false);
/// assert_eq!(sorted[0], "กาล");
/// ```
pub fn collate(words: &[&str], reverse: bool) -> Vec<String> {
    let mut sorted: Vec<&str> = words.to_vec();
    sorted.sort_by_key(|w| thai_sort_key(w));
    if reverse {
        sorted.reverse();
    }
    sorted.iter().map(|s| s.to_string()).collect()
}

// ---------------------------------------------------------------------------
// Digit conversion
// ---------------------------------------------------------------------------

/// Convert Thai digit characters (๐-๙) to Arabic digits (0-9).
///
/// # Examples
/// ```
/// use agi_thainlp::thai_digit_to_arabic_digit;
/// assert_eq!(thai_digit_to_arabic_digit("๑๒๓"), "123");
/// ```
pub fn thai_digit_to_arabic_digit(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '๐' => '0',
            '๑' => '1',
            '๒' => '2',
            '๓' => '3',
            '๔' => '4',
            '๕' => '5',
            '๖' => '6',
            '๗' => '7',
            '๘' => '8',
            '๙' => '9',
            _ => c,
        })
        .collect()
}

/// Convert Arabic digit characters (0-9) to Thai digits (๐-๙).
///
/// # Examples
/// ```
/// use agi_thainlp::arabic_digit_to_thai_digit;
/// assert_eq!(arabic_digit_to_thai_digit("123"), "๑๒๓");
/// ```
pub fn arabic_digit_to_thai_digit(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '0' => '๐',
            '1' => '๑',
            '2' => '๒',
            '3' => '๓',
            '4' => '๔',
            '5' => '๕',
            '6' => '๖',
            '7' => '๗',
            '8' => '๘',
            '9' => '๙',
            _ => c,
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Number to Thai word
// ---------------------------------------------------------------------------

const ONES: &[&str] = &[
    "ศูนย์", "หนึ่ง", "สอง", "สาม", "สี่",
    "ห้า", "หก", "เจ็ด", "แปด", "เก้า",
];
const TENS: &str = "สิบ";
const TENS_TWO: &str = "ยี่สิบ";
const HUNDRED: &str = "ร้อย";
const THOUSAND: &str = "พัน";
const TEN_THOUSAND: &str = "หมื่น";
const HUNDRED_THOUSAND: &str = "แสน";
const MILLION: &str = "ล้าน";
const ET: &str = "เอ็ด"; // "one" in final position

fn chunk_to_thaiword(n: u64) -> String {
    if n == 0 {
        return String::new();
    }
    let mut s = String::new();
    let h_t = (n / 100_000) as usize;
    let n = n % 100_000;
    let tm = (n / 10_000) as usize;
    let n = n % 10_000;
    let th = (n / 1_000) as usize;
    let n = n % 1_000;
    let hu = (n / 100) as usize;
    let n = n % 100;
    let te = (n / 10) as usize;
    let on = (n % 10) as usize;

    if h_t > 0 { s.push_str(&format!("{}{}", ONES[h_t], HUNDRED_THOUSAND)); }
    if tm > 0 { s.push_str(&format!("{}{}", ONES[tm], TEN_THOUSAND)); }
    if th > 0 { s.push_str(&format!("{}{}", ONES[th], THOUSAND)); }
    if hu > 0 { s.push_str(&format!("{}{}", ONES[hu], HUNDRED)); }
    if te > 0 {
        if te == 2 {
            s.push_str(TENS_TWO);
        } else if te == 1 {
            s.push_str(TENS);
        } else {
            s.push_str(&format!("{}{}", ONES[te], TENS));
        }
    }
    if on > 0 {
        if on == 1 && te > 0 {
            s.push_str(ET);
        } else {
            s.push_str(ONES[on]);
        }
    }
    s
}

/// Convert a non-negative integer to its Thai word representation.
///
/// # Examples
/// ```
/// use agi_thainlp::num_to_thaiword;
/// assert_eq!(num_to_thaiword(0), "ศูนย์");
/// assert_eq!(num_to_thaiword(21), "ยี่สิบเอ็ด");
/// assert_eq!(num_to_thaiword(1_000_000), "หนึ่งล้าน");
/// ```
pub fn num_to_thaiword(n: u64) -> String {
    if n == 0 {
        return ONES[0].to_string();
    }
    let millions = n / 1_000_000;
    let remainder = n % 1_000_000;
    let mut s = String::new();
    if millions > 0 {
        s.push_str(&chunk_to_thaiword(millions));
        s.push_str(MILLION);
    }
    if remainder > 0 {
        s.push_str(&chunk_to_thaiword(remainder));
    }
    s
}

// ---------------------------------------------------------------------------
// Thai word to number
// ---------------------------------------------------------------------------

/// Convert a Thai number word to an integer, or return None on failure.
///
/// # Examples
/// ```
/// use agi_thainlp::thaiword_to_num;
/// assert_eq!(thaiword_to_num("ยี่สิบเอ็ด"), Some(21));
/// assert_eq!(thaiword_to_num("หนึ่งร้อยห้าสิบ"), Some(150));
/// ```
pub fn thaiword_to_num(text: &str) -> Option<u64> {
    if text.is_empty() {
        return None;
    }

    // Token table: (word, value, is_multiplier)
    let token_table: &[(&str, u64, bool)] = &[
        ("ศูนย์", 0, false),
        ("หนึ่ง", 1, false),
        ("สอง",   2, false),
        ("สาม",   3, false),
        ("สี่",   4, false),
        ("ห้า",   5, false),
        ("หก",    6, false),
        ("เจ็ด",  7, false),
        ("แปด",   8, false),
        ("เก้า",  9, false),
        ("เอ็ด",  1, false),
        ("ยี่",   2, false),
        ("สิบ",        10, true),
        ("ร้อย",       100, true),
        ("พัน",        1_000, true),
        ("หมื่น",      10_000, true),
        ("แสน",        100_000, true),
        ("ล้าน",       1_000_000, true),
    ];

    // Sort by token length descending for greedy match
    let mut sorted: Vec<(&str, u64, bool)> = token_table.to_vec();
    sorted.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Tokenize
    let mut tokens: Vec<(u64, bool)> = Vec::new();
    let mut pos = 0;
    let n = text.len();
    while pos < n {
        let slice = &text[pos..];
        let mut matched = false;
        for (word, val, is_mult) in &sorted {
            if slice.starts_with(word) {
                tokens.push((*val, *is_mult));
                pos += word.len();
                matched = true;
                break;
            }
        }
        if !matched {
            return None;
        }
    }

    // Parse hierarchically
    let mut total: u64 = 0;
    let mut group: u64 = 0;
    let mut pending: u64 = 0;

    for (val, is_mult) in tokens {
        if !is_mult {
            pending += val;
        } else if val == 1_000_000 {
            let group_val = if group > 0 || pending > 0 { group + pending } else { 1 };
            total += group_val * 1_000_000;
            group = 0;
            pending = 0;
        } else {
            let factor = if pending > 0 { pending } else { 1 };
            group += factor * val;
            pending = 0;
        }
    }

    Some(total + group + pending)
}

// ---------------------------------------------------------------------------
// Normalize — remove tonemarks
// ---------------------------------------------------------------------------

static RE_TONEMARK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\u{0e48}\u{0e49}\u{0e4a}\u{0e4b}]").unwrap()
});

/// Remove Thai tonemarks from text.
///
/// # Examples
/// ```
/// use agi_thainlp::remove_tonemark;
/// assert_eq!(remove_tonemark("น้ำ"), "นำ");
/// ```
pub fn remove_tonemark(text: &str) -> String {
    RE_TONEMARK.replace_all(text, "").to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collate_basic() {
        let words = vec!["ไก่", "เกิด", "กาล"];
        let sorted = collate(&words, false);
        assert_eq!(sorted[0], "กาล");
    }

    #[test]
    fn collate_reverse() {
        let words = vec!["ก", "ข", "ค"];
        let sorted = collate(&words, true);
        assert_eq!(sorted[0], "ค");
    }

    #[test]
    fn thai_to_arabic() {
        assert_eq!(thai_digit_to_arabic_digit("๑๒๓"), "123");
    }

    #[test]
    fn arabic_to_thai() {
        assert_eq!(arabic_digit_to_thai_digit("123"), "๑๒๓");
    }

    #[test]
    fn num_zero() {
        assert_eq!(num_to_thaiword(0), "ศูนย์");
    }

    #[test]
    fn num_ten() {
        assert_eq!(num_to_thaiword(10), "สิบ");
    }

    #[test]
    fn num_twenty_one() {
        assert_eq!(num_to_thaiword(21), "ยี่สิบเอ็ด");
    }

    #[test]
    fn num_million() {
        assert_eq!(num_to_thaiword(1_000_000), "หนึ่งล้าน");
    }

    #[test]
    fn thaiword_twenty_one() {
        assert_eq!(thaiword_to_num("ยี่สิบเอ็ด"), Some(21));
    }

    #[test]
    fn thaiword_hundred_fifty() {
        assert_eq!(thaiword_to_num("หนึ่งร้อยห้าสิบ"), Some(150));
    }

    #[test]
    fn remove_tone_basic() {
        assert_eq!(remove_tonemark("น้ำ"), "นำ");
        assert_eq!(remove_tonemark("ข้าว"), "ขาว");
    }
}
