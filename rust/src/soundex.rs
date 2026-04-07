//! Thai soundex systems: Udom83 and LK82.
//!
//! * **Udom83**: Wannee Udompanich (1983), 7-character code.
//! * **LK82**: Vichit Lorchirachoonkul (1982), 5-character code.

use once_cell::sync::Lazy;
use regex::Regex;

const THAI_CONSONANTS: &str = "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ";

// ---------------------------------------------------------------------------
// Udom83
// ---------------------------------------------------------------------------

static U83_RE_1: Lazy<Regex> = Lazy::new(|| Regex::new(r"รร([\u{0e40}-\u{0e44}])").unwrap());
static U83_RE_2: Lazy<Regex> = Lazy::new(|| {
    // รร followed by Thai consonant + (Thai consonant or lead vowel เ-ไ)
    Regex::new(r"รร([ก-ฮ][ก-ฮเแโใไ])").unwrap()
});
static U83_RE_3: Lazy<Regex> = Lazy::new(|| {
    // รร followed by Thai consonant + (vowel diacritics or tonemarks)
    Regex::new(r"รร([ก-ฮ][ะ-ูึ-ๅ่-์])").unwrap()
});
static U83_RE_4: Lazy<Regex> = Lazy::new(|| Regex::new(r"รร").unwrap());
static U83_RE_5: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!("ไ([{THAI_CONSONANTS}]ย)")).unwrap()
});
static U83_RE_6: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!("[ไใ]([{THAI_CONSONANTS}])")).unwrap()
});
static U83_RE_7: Lazy<Regex> = Lazy::new(|| Regex::new(r"\u{0e33}(ม[\u{0e30}-\u{0e39}])").unwrap());
static U83_RE_8: Lazy<Regex> = Lazy::new(|| Regex::new(r"\u{0e33}ม").unwrap());
static U83_RE_9: Lazy<Regex> = Lazy::new(|| Regex::new(r"\u{0e33}").unwrap());
static U83_RE_10: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&format!(
        r"จน์|มณ์|ณฑ์|ทร์|ตร์|[{THAI_CONSONANTS}]\u{{0e4c}}|[{THAI_CONSONANTS}][\u{{0e30}}-\u{{0e39}}]\u{{0e4c}}"
    )).unwrap()
});
static U83_RE_11: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\u{0e30}-\u{0e4c}]").unwrap());

fn u83_translate1(c: char) -> char {
    match c {
        'ก' => 'ก', 'ข'|'ฃ'|'ค'|'ฅ'|'ฆ' => 'ข',
        'ง' => 'ง',
        'จ' => 'จ',
        'ฉ'|'ช'|'ฌ' => 'ช',
        'ซ'|'ศ'|'ษ'|'ส' => 'ส',
        'ฎ'|'ด' => 'ด',
        'ฏ'|'ต' => 'ต',
        'ฐ'|'ฑ'|'ฒ'|'ถ'|'ท'|'ธ' => 'ท',
        'ณ'|'น' => 'น',
        'บ' => 'บ',
        'ป' => 'ป',
        'ผ'|'พ'|'ภ' => 'พ',
        'ฝ'|'ฟ' => 'ฟ',
        'ม' => 'ม',
        'ญ'|'ย' => 'ย',
        'ร'|'ล'|'ฬ'|'ฤ'|'ฦ' => 'ร',
        'ว' => 'ว',
        'อ' => 'อ',
        'ห'|'ฮ' => 'ฮ',
        _ => c,
    }
}

fn u83_translate2(c: char) -> char {
    match c {
        'ม'|'ว'|'ำ' => '0',
        'ก'|'ข'|'ฃ'|'ค'|'ฅ'|'ฆ' => '1',
        'ง' => '2',
        'ย'|'ญ' => '2',
        'ณ'|'น' => '3',
        'ฎ'|'ฏ'|'ด'|'ต' => '4',
        'ศ'|'ษ'|'ส' => '4',
        'บ'|'ป' => '5',
        'พ'|'ภ'|'ผ' => '5',
        'ฝ'|'ฟ' => '5',
        'ห'|'อ'|'ฮ' => '6',
        'จ'|'ฉ'|'ช'|'ซ'|'ฌ' => '7',
        'ฐ'|'ฑ'|'ฒ'|'ถ'|'ท'|'ธ' => '8',
        'ร'|'ฤ'|'ล'|'ฦ' => '9',
        _ => c,
    }
}

/// Convert Thai text to Udom83 soundex code (7 characters).
///
/// # Examples
/// ```
/// use agi_thainlp::udom83;
/// assert_eq!(udom83("รัก"), "ร100000");
/// assert_eq!(udom83("ลัก"), "ร100000");
/// ```
pub fn udom83(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let s = U83_RE_1.replace_all(text, "ัน$1").to_string();
    let s = U83_RE_2.replace_all(&s, "ั$1").to_string();
    let s = U83_RE_3.replace_all(&s, "ัน$1").to_string();
    let s = U83_RE_4.replace_all(&s, "ัน").to_string();
    let s = U83_RE_5.replace_all(&s, "$1").to_string();
    let s = U83_RE_6.replace_all(&s, "${1}ย").to_string();
    let s = U83_RE_7.replace_all(&s, "ม$1").to_string();
    let s = U83_RE_8.replace_all(&s, "ม").to_string();
    let s = U83_RE_9.replace_all(&s, "ม").to_string();
    let s = U83_RE_10.replace_all(&s, "").to_string();
    let s = U83_RE_11.replace_all(&s, "").to_string();

    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();
    let first_encoded = u83_translate1(first);

    let rest_encoded: String = chars.map(u83_translate2).collect();

    let code = format!("{first_encoded}{rest_encoded}000000");
    code[..code
        .char_indices()
        .nth(7)
        .map(|(i, _)| i)
        .unwrap_or(code.len())]
        .to_string()
}

// ---------------------------------------------------------------------------
// LK82
// ---------------------------------------------------------------------------

static LK82_RE_KARANT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"จน์|มณ์|ณฑ์|ทร์|ตร์|[ก-ฮ]์|[ก-ฮ][ะ-ู]์").unwrap()
});
static LK82_RE_SIGN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\u{0e2f}\u{0e3a}\u{0e46}\u{0e47}\u{0e4d}]").unwrap()
});
static LK82_RE_TONE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\u{0e48}\u{0e49}\u{0e4a}\u{0e4b}]").unwrap()
});

fn lk82_trans1(c: char) -> char {
    match c {
        'ก'|'ข'|'ฃ'|'ค'|'ฅ'|'ฆ' => 'ก',
        'ง' => 'ง',
        'จ' => 'จ',
        'ฉ'|'ช'|'ฌ' => 'ช',
        'ซ'|'ศ'|'ษ'|'ส' => 'ซ',
        'ญ'|'ย' => 'ย',
        'ฎ'|'ด' => 'ด',
        'ฏ'|'ต' => 'ต',
        'ณ'|'น' => 'น',
        'ฐ'|'ฑ'|'ฒ'|'ถ'|'ท'|'ธ' => 'ท',
        'บ'|'ป' => 'บ',
        'ผ'|'พ'|'ภ' => 'พ',
        'ฝ'|'ฟ' => 'ฟ',
        'ม' => 'ม',
        'ร'|'ล'|'ฬ'|'ฤ'|'ฦ' => 'ร',
        'ว' => 'ว',
        'ห'|'ฮ' => 'ห',
        'อ' => 'อ',
        _ => c,
    }
}

fn lk82_trans2(c: char) -> char {
    match c {
        'ก'|'ข'|'ฃ'|'ค'|'ฅ'|'ฆ' => '1',
        'ง' => '2',
        'จ'|'ฉ'|'ช'|'ซ'|'ฌ' => '3',
        'ฎ'|'ฏ'|'ฐ'|'ฑ'|'ฒ'|'ด'|'ต'|'ถ'|'ท'|'ธ' => '3',
        'ศ'|'ษ'|'ส' => '3',
        'ญ'|'ณ'|'น' => '4',
        'ร'|'ล'|'ฬ'|'ฤ'|'ฦ' => '4',
        'บ'|'ป'|'พ'|'ฟ'|'ภ'|'ผ'|'ฝ' => '5',
        'ม' => '6',
        'ำ' => '6',
        'ย'|'ว' => '7',
        'ไ'|'ใ' => '7',
        'ห'|'ฮ' => '8',
        'า'|'ๅ' => '9',
        'ึ'|'ื' => 'A',
        'เ'|'แ' => 'B',
        'โ' => 'C',
        'ุ'|'ู' => 'D',
        'อ' => 'E',
        _ => c,
    }
}

/// Convert Thai text to LK82 soundex code (5 characters).
///
/// # Examples
/// ```
/// use agi_thainlp::lk82;
/// assert_eq!(lk82("รัก"), "ร1000");
/// assert_eq!(lk82("ลัก"), "ร1000");
/// ```
pub fn lk82(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let s = LK82_RE_TONE.replace_all(text, "").to_string();
    let s = LK82_RE_KARANT.replace_all(&s, "").to_string();
    let s = LK82_RE_SIGN.replace_all(&s, "").to_string();

    if s.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut res: Vec<String> = Vec::new();

    // Encode first character
    let start;
    if chars[0] >= 'ก' && chars[0] <= 'ฮ' {
        res.push(lk82_trans1(chars[0]).to_string());
        start = 1;
    } else {
        if n > 1 {
            res.push(lk82_trans1(chars[1]).to_string());
        }
        res.push(lk82_trans2(chars[0]).to_string());
        start = 2;
    }

    let mut i_v: Option<usize> = None;
    for (i, &c) in chars[start..].iter().enumerate() {
        let i = i + start; // actual index
        match c {
            '\u{0e30}'|'\u{0e31}'|'\u{0e34}'|'\u{0e35}' => {
                // Sara A, Mai Han-Akat, Sara I, Sara Ii — separator only
                i_v = Some(i);
                res.push(String::new());
            }
            '\u{0e32}'|'\u{0e36}'|'\u{0e37}'|'\u{0e39}'|'\u{0e45}' => {
                // Sara Aa, Sara Ue, Sara Uee, Sara Uu, Lak Khang Yao — separator + encoded
                i_v = Some(i);
                res.push(lk82_trans2(c).to_string());
            }
            '\u{0e38}' => { // Sara U
                i_v = Some(i);
                if i == start || (chars[i - 1] != 'ต' && chars[i - 1] != 'ธ') {
                    res.push(lk82_trans2(c).to_string());
                } else {
                    res.push(String::new());
                }
            }
            '\u{0e2b}'|'\u{0e2d}' => { // ห อ
                if i + 1 < n && "\u{0e36}\u{0e37}\u{0e38}\u{0e39}".contains(chars[i + 1]) {
                    res.push(lk82_trans2(c).to_string());
                }
            }
            '\u{0e22}'|'\u{0e23}'|'\u{0e24}'|'\u{0e26}'|'\u{0e27}' => {
                // ย ร ฤ ฦ ว
                let prev_is_vowel = i_v.map_or(false, |iv| iv == i - 1);
                let next_is_ue = i + 1 < n && "\u{0e36}\u{0e37}\u{0e38}\u{0e39}".contains(chars[i + 1]);
                if prev_is_vowel || next_is_ue {
                    res.push(lk82_trans2(c).to_string());
                }
            }
            _ => {
                res.push(lk82_trans2(c).to_string());
            }
        }
    }

    // Remove consecutive duplicates
    let mut deduped: Vec<String> = Vec::new();
    for item in res {
        if deduped.last().map_or(true, |last| *last != item) {
            deduped.push(item);
        }
    }

    let combined: String = deduped.join("") + "0000";
    // Take first 5 characters
    combined
        .char_indices()
        .take(5)
        .map(|(_, c)| c)
        .collect::<String>()
}

/// Convert Thai text to soundex code using the specified engine.
///
/// # Arguments
/// * `text` — Thai word
/// * `engine` — `"udom83"` (default) or `"lk82"`
///
/// # Examples
/// ```
/// use agi_thainlp::soundex;
/// assert_eq!(soundex("รัก", "udom83"), "ร100000");
/// assert_eq!(soundex("รัก", "lk82"), "ร1000");
/// ```
pub fn soundex(text: &str, engine: &str) -> String {
    match engine {
        "lk82" => lk82(text),
        _ => udom83(text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn udom83_rak() {
        assert_eq!(udom83("รัก"), "ร100000");
    }

    #[test]
    fn udom83_lak() {
        assert_eq!(udom83("ลัก"), "ร100000");
    }

    #[test]
    fn udom83_length() {
        assert_eq!(udom83("บูรณการ").chars().count(), 7);
    }

    #[test]
    fn udom83_empty() {
        assert_eq!(udom83(""), "");
    }

    #[test]
    fn lk82_rak() {
        assert_eq!(lk82("รัก"), "ร1000");
    }

    #[test]
    fn lk82_lak() {
        assert_eq!(lk82("ลัก"), "ร1000");
    }

    #[test]
    fn lk82_length() {
        assert_eq!(lk82("บูรณการ").chars().count(), 5);
    }

    #[test]
    fn lk82_empty() {
        assert_eq!(lk82(""), "");
    }

    #[test]
    fn soundex_default_udom83() {
        assert_eq!(soundex("รัก", "udom83"), udom83("รัก"));
    }

    #[test]
    fn soundex_lk82() {
        assert_eq!(soundex("รัก", "lk82"), lk82("รัก"));
    }
}
