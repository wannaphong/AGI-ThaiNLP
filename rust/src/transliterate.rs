//! RTGS (Royal Thai General System of Transcription) romanization.
//!
//! Converts Thai text to Latin alphabet using the official RTGS system
//! published by the Royal Institute of Thailand.
//!
//! This implementation uses TCC tokenization and does not rely on any
//! external language resources.
//!
//! Reference:
//!   https://en.wikipedia.org/wiki/Royal_Thai_General_System_of_Transcription

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::tokenize::tcc_tokenize;

const THAI_CONSONANTS: &str = "กขฃคฅฆงจฉชซฌญฎฏฐฑฒณดตถทธนบปผฝพฟภมยรลวศษสหฬอฮ";

static RE_CONSONANT: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[ก-ฮ]").expect("consonant regex failed")
});

static RE_NORMALIZE: Lazy<Regex> = Lazy::new(|| {
    let thanthakhat = "\u{0e4c}";
    let c = r"[ก-ฮ]";
    let pattern = format!(
        r"จน์|มณ์|ณฑ์|ทร์|ตร์|{c}{thanthakhat}|{c}[\u{{0e30}}-\u{{0e39}}]{thanthakhat}|[\u{{0e2f}}\u{{0e46}}\u{{0e48}}-\u{{0e4f}}\u{{0e5a}}\u{{0e5b}}]"
    );
    Regex::new(&pattern).expect("normalize regex failed")
});

// Vowel substitution patterns: (Thai pattern, romanized replacement)
static VOWEL_PATTERNS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    let c = r"([ก-ฮ])";
    let cn = r"([คนพมห])";
    let ck = r"([กตทปศส])";
    let raw: &[(&str, &str)] = &[
        (r"เCียว", r"${1}iao"),
        (r"แC็ว",  r"${1}aeo"),
        (r"เCือย", r"${1}ueai"),
        (r"แCว",   r"${1}aeo"),
        (r"เC็ว",  r"${1}eo"),
        (r"เCว",   r"${1}eo"),
        (r"Cิว",   r"${1}io"),
        (r"Cวย",   r"${1}uai"),
        (r"เCย",   r"${1}oei"),
        (r"Cอย",   r"${1}oi"),
        (r"โCย",   r"${1}oi"),
        (r"Cุย",   r"${1}ui"),
        (r"Cาย",   r"${1}ai"),
        (r"ไCย",   r"${1}ai"),
        (r"Cัย",   r"${1}ai"),
        (r"ไCC",   r"${1}${2}ai"),
        (r"ไC",    r"${1}ai"),
        (r"ใC",    r"${1}ai"),
        (r"CวC",   r"${1}ua${2}"),
        (r"Cัวะ",  r"${1}ua"),
        (r"Cัว",   r"${1}ua"),
        (r"เCือะ", r"${1}uea"),
        (r"เCือ",  r"${1}uea"),
        (r"เCียะ", r"${1}ia"),
        (r"เCีย",  r"${1}ia"),
        (r"เCอะ",  r"${1}oe"),
        (r"เCอ",   r"${1}oe"),
        (r"เCิ",   r"${1}oe"),
        (r"Cอ",    r"${1}o"),
        (r"เCาะ",  r"${1}o"),
        (r"เC็",   r"${1}e"),
        (r"โCะ",   r"${1}o"),
        (r"โC",    r"${1}o"),
        (r"แCะ",   r"${1}ae"),
        (r"แC",    r"${1}ae"),
        (r"เCาะ",  r"${1}e"),
        (r"Cาว",   r"${1}ao"),
        (r"เCา",   r"${1}ao"),
        (r"เC",    r"${1}e"),
        (r"Cู",    r"${1}u"),
        (r"Cุ",    r"${1}u"),
        (r"Cื",    r"${1}ue"),
        (r"Cึ",    r"${1}ue"),
        (r"Cี",    r"${1}i"),
        (r"Cิ",    r"${1}i"),
        (r"Cำ",    r"${1}am"),
        (r"Cา",    r"${1}a"),
        (r"Cั",    r"${1}a"),
        (r"Cะ",    r"${1}a"),
        (r"CNฤ",    r"${1}${2}rue"),  // # pattern — CN = ([คนพมห])
        (r"CKฤ",    r"${1}${2}ri"),   // $ pattern — CK = ([กตทปศส])
    ];

    raw.iter().map(|(pattern, replacement)| {
        let p = pattern
            .replace("CN", &format!("({cn})({c})"))
            .replace("CK", &format!("({ck})({c})"))
            .replace("CC", &format!("{c}{c}"))
            .replace('C', c);
        (
            Regex::new(&p).unwrap_or_else(|e| panic!("vowel pattern {p} failed: {e}")),
            *replacement,
        )
    }).collect()
});

// Initial and final consonant romanizations
fn consonants_map() -> &'static HashMap<char, [&'static str; 2]> {
    static MAP: Lazy<HashMap<char, [&'static str; 2]>> = Lazy::new(|| {
        let entries: &[(char, &str, &str)] = &[
            ('ก', "k",  "k"),
            ('ข', "kh", "k"),
            ('ฃ', "kh", "k"),
            ('ค', "kh", "k"),
            ('ฅ', "kh", "k"),
            ('ฆ', "kh", "k"),
            ('ง', "ng", "ng"),
            ('จ', "ch", "t"),
            ('ฉ', "ch", "t"),
            ('ช', "ch", "t"),
            ('ซ', "s",  "t"),
            ('ฌ', "ch", "t"),
            ('ญ', "y",  "n"),
            ('ฎ', "d",  "t"),
            ('ฏ', "t",  "t"),
            ('ฐ', "th", "t"),
            ('ฑ', "th", "t"),
            ('ฒ', "th", "t"),
            ('ณ', "n",  "n"),
            ('ด', "d",  "t"),
            ('ต', "t",  "t"),
            ('ถ', "th", "t"),
            ('ท', "th", "t"),
            ('ธ', "th", "t"),
            ('น', "n",  "n"),
            ('บ', "b",  "p"),
            ('ป', "p",  "p"),
            ('ผ', "ph", "p"),
            ('ฝ', "f",  "p"),
            ('พ', "ph", "p"),
            ('ฟ', "f",  "p"),
            ('ภ', "ph", "p"),
            ('ม', "m",  "m"),
            ('ย', "y",  ""),
            ('ร', "r",  "n"),
            ('ฤ', "rue",""),
            ('ล', "l",  "n"),
            ('ว', "w",  ""),
            ('ศ', "s",  "t"),
            ('ษ', "s",  "t"),
            ('ส', "s",  "t"),
            ('ห', "h",  ""),
            ('ฬ', "l",  "n"),
            ('อ', "",   ""),
            ('ฮ', "h",  ""),
        ];
        entries.iter().map(|(c, init, fin)| (*c, [*init, *fin])).collect()
    });
    &MAP
}

fn normalize(word: &str) -> String {
    RE_NORMALIZE.replace_all(word, "").to_string()
}

fn replace_vowels(word: &str) -> String {
    let mut s = word.to_string();
    for (re, replacement) in VOWEL_PATTERNS.iter() {
        s = re.replace_all(&s, *replacement).to_string();
    }
    s
}

fn is_thai_consonant(c: char) -> bool {
    THAI_CONSONANTS.contains(c)
}

const ROMANIZED_VOWELS: &str = "aeiou";
const HO_HIP: char = '\u{0e2b}';   // ห
const RO_RUA: char = '\u{0e23}';   // ร
const LO_LING: char = '\u{0e25}';  // ล
const WO_WAEN: char = '\u{0e27}';  // ว

fn replace_consonants(word: &str, consonants: &[char]) -> String {
    let cmap = consonants_map();
    let word_chars: Vec<char> = word.chars().collect();
    let n = word_chars.len();
    let mut result: Vec<String> = Vec::new();
    let mut j = 0usize; // index into consonants slice
    let mut vowel_seen = false;
    let mut skip = false;

    for i in 0..n {
        if skip {
            skip = false;
            j += 1;
            continue;
        }

        let ch = word_chars[i];

        if !is_thai_consonant(ch) {
            vowel_seen = true;
            result.push(ch.to_string());
        } else if result.is_empty() && ch == HO_HIP && consonants.len() != 1 {
            j += 1; // skip leading ห in cluster
        } else if i + 1 < n && word_chars[i] == RO_RUA && word_chars[i + 1] == RO_RUA
            && i + 2 == n
        {
            // รร at end of word → an
            skip = true;
            result.push("an".to_string());
            vowel_seen = true;
            j += 1;
        } else if i + 1 < n && word_chars[i] == RO_RUA && word_chars[i + 1] == RO_RUA {
            // รร in middle → a
            skip = true;
            result.push("a".to_string());
            vowel_seen = true;
            j += 1;
        } else if !vowel_seen {
            let has_initial = result.iter().any(|s| {
                s.chars().any(|c| !ROMANIZED_VOWELS.contains(c))
            });

            if !has_initial {
                if let Some(mapping) = cmap.get(&consonants[j]) {
                    if !mapping[0].is_empty() {
                        result.push(mapping[0].to_string());
                    }
                }
                j += 1;
            } else {
                let is_cluster = ch == RO_RUA || ch == LO_LING || ch == WO_WAEN;
                let is_last = i + 1 >= n;
                let next_is_vowel = !is_last && !is_thai_consonant(word_chars[i + 1]);

                if is_cluster && (next_is_vowel || !is_last) {
                    if let Some(mapping) = cmap.get(&consonants[j]) {
                        result.push(mapping[0].to_string());
                    }
                    j += 1;
                } else if !is_cluster && !is_last {
                    result.push("a".to_string());
                    if let Some(mapping) = cmap.get(&consonants[j]) {
                        if !mapping[0].is_empty() {
                            result.push(mapping[0].to_string());
                        }
                    }
                    vowel_seen = false;
                    j += 1;
                } else if next_is_vowel {
                    if let Some(mapping) = cmap.get(&consonants[j]) {
                        result.push(mapping[0].to_string());
                    }
                    j += 1;
                } else if is_last {
                    result.push("o".to_string());
                    if let Some(mapping) = cmap.get(&consonants[j]) {
                        result.push(mapping[1].to_string());
                    }
                    vowel_seen = true;
                    j += 1;
                } else {
                    result.push("o".to_string());
                    if let Some(mapping) = cmap.get(&consonants[j]) {
                        result.push(mapping[1].to_string());
                    }
                    vowel_seen = true;
                    j += 1;
                }
            }
        } else {
            let next_is_vowel = i + 1 < n && !is_thai_consonant(word_chars[i + 1]);
            if next_is_vowel {
                if let Some(mapping) = cmap.get(&consonants[j]) {
                    result.push(mapping[0].to_string());
                }
                vowel_seen = false;
                j += 1;
            } else {
                if let Some(mapping) = cmap.get(&consonants[j]) {
                    result.push(mapping[1].to_string());
                }
                j += 1;
            }
        }
    }

    result.join("")
}

fn romanize_word(word: &str) -> String {
    if word == "ห" {
        return String::new();
    }
    let normalized = normalize(word);
    let mut vowels_replaced = replace_vowels(&normalized);

    let consonants: Vec<char> = RE_CONSONANT
        .find_iter(&vowels_replaced)
        .map(|m| m.as_str().chars().next().unwrap())
        .collect();

    // Two-character all-consonant word: insert implicit 'o'
    let char_count = vowels_replaced.chars().count();
    if char_count == 2 && consonants.len() == 2 {
        let mut chars = vowels_replaced.chars();
        let c1 = chars.next().unwrap();
        let c2 = chars.next().unwrap();
        vowels_replaced = format!("{c1}o{c2}");
    }

    replace_consonants(&vowels_replaced, &consonants)
}

/// Romanize Thai text using the Royal Thai General System of Transcription (RTGS).
///
/// Non-Thai characters are passed through unchanged.
///
/// # Examples
/// ```
/// use agi_thainlp::romanize;
/// let r = romanize("กาแฟ");
/// assert!(r.contains('k'));
/// ```
pub fn romanize(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    while i < n {
        let ch = chars[i];
        if ('\u{0e00}'..='\u{0e7f}').contains(&ch) {
            // Collect Thai run
            let start = i;
            while i < n && ('\u{0e00}'..='\u{0e7f}').contains(&chars[i]) {
                i += 1;
            }
            let thai_run: String = chars[start..i].iter().collect();
            // Tokenize into TCC units and romanize each
            for token in tcc_tokenize(&thai_run) {
                result.push_str(&romanize_word(&token));
            }
        } else {
            result.push(ch);
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanize_empty() {
        assert_eq!(romanize(""), "");
    }

    #[test]
    fn romanize_non_thai() {
        assert_eq!(romanize("hello"), "hello");
    }

    #[test]
    fn romanize_basic_ka() {
        let r = romanize("กา");
        assert!(r.contains('k'), "expected 'k' in '{r}'");
    }

    #[test]
    fn romanize_mixed() {
        let r = romanize("กา hello");
        assert!(r.contains("hello"));
    }

    #[test]
    fn romanize_kafae() {
        // กาแฟ → kafae
        let r = romanize("กาแฟ");
        assert!(r.contains('k') && r.contains('f'), "got: {r}");
    }
}
