//! Thai tokenization: TCC (Thai Character Cluster) and sentence splitting.
//!
//! The TCC algorithm is based on:
//!   Theeramunkong, T. et al. (2000). Character cluster based Thai
//!   information retrieval. IRAL 2000.

use once_cell::sync::Lazy;
use regex::Regex;

// ็ is U+0E47, ์ is U+0E4C
// Thai consonants: ก-ฮ (U+0E01–U+0E2E)
// Thai lead vowels: เ-ไ (U+0E40–U+0E44)
// Thai tonemarks: ่-๋ (U+0E48–U+0E4B)

// The `k` ending cluster suffix (optional final consonant cluster):
//   (CC?[ิุู]?[์])?  where C = [ก-ฮ]
// Using literal characters to avoid escape issues.
const K_SUFFIX: &str = r"(?:[ก-ฮ][ก-ฮ]?[ิุู]?[์])?";

/// TCC regex patterns based on the Theeramunkong et al. 2000 rules.
static TCC_PATTERN: Lazy<Regex> = Lazy::new(|| {
    let k = K_SUFFIX;
    let c = r"[ก-ฮ]";
    let t = r"[่-๋]?";

    // Patterns with placeholders for c, t, k (substituted below)
    let raw_patterns: &[String] = &[
        format!("{c}[ั]([่-๋]{c})?"),
        format!("{c}[ั]([่-๋]{c})?{k}"),
        format!("เ{c}็{c}{k}"),
        format!("เ{c}{c}{t}าะ{k}"),
        format!("เ{c}{c}ี{t}ยะ{k}"),
        // Removed lookahead (?=[เ-ไก-ฮ]|$) — not supported by regex crate
        format!("เ{c}{c}ี{t}ย{k}"),
        format!("เ{c}[ิีุู]{t}ย{k}"),
        format!("เ{c}{c}็{c}{k}"),
        format!("เ{c}ิ{c}์{c}{k}"),
        format!("เ{c}ิ{t}{c}{k}"),
        format!("เ{c}ี{t}ยะ?{k}"),
        format!("เ{c}ื{t}อะ{k}"),
        format!("เ{c}ื"),
        format!("เ{c}{t}า?ะ?{k}"),
        format!("{c}[ึื]{t}{c}{k}"),
        format!("{c}[ะ-ู]{t}{k}"),
        format!("{c}[ิุู]์"),
        format!("{c}รร{c}์"),
        format!("{c}็"),
        format!("{c}{t}[ะาำ]?{k}"),
        format!("แ{c}็{c}{k}"),
        format!("แ{c}{c}์{k}"),
        format!("แ{c}{t}ะ{k}"),
        format!("แ{c}{c}็{c}{k}"),
        format!("แ{c}{c}{c}์{k}"),
        format!("โ{c}{t}ะ{k}"),
        format!("[เ-ไ]{c}{t}{k}"),
        "ก็".to_string(),
        "อึ".to_string(),
        "หึ".to_string(),
    ];

    let combined = raw_patterns.join("|");
    Regex::new(&combined).expect("TCC regex compilation failed")
});

/// Tokenize Thai text into Thai Character Clusters (TCC).
///
/// A Thai Character Cluster (TCC) is the smallest indivisible unit for
/// Thai text processing, roughly corresponding to a syllable nucleus.
///
/// # Examples
/// ```
/// use agi_thainlp::tcc_tokenize;
/// let tokens = tcc_tokenize("กาแฟ");
/// assert!(tokens.contains(&"กา".to_string()));
/// assert!(tokens.contains(&"แฟ".to_string()));
/// ```
pub fn tcc_tokenize(text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let n = text.len();
    let mut byte_pos = 0;

    while byte_pos < n {
        let slice = &text[byte_pos..];
        if let Some(m) = TCC_PATTERN.find(slice) {
            if m.start() == 0 {
                result.push(m.as_str().to_string());
                byte_pos += m.end();
            } else {
                // No match at current position — emit one char
                let ch = slice.chars().next().unwrap();
                result.push(ch.to_string());
                byte_pos += ch.len_utf8();
            }
        } else {
            // No match anywhere — emit one char
            let ch = slice.chars().next().unwrap();
            result.push(ch.to_string());
            byte_pos += ch.len_utf8();
        }
    }
    result
}

/// Tokenize Thai text into syllables (alias for [`tcc_tokenize`]).
pub fn syllable_tokenize(text: &str) -> Vec<String> {
    tcc_tokenize(text)
}

/// Split text into sentences.
///
/// Splits on:
/// * ASCII `.!?` followed by a space and then a capital letter or Thai character.
/// * Thai sentence-end markers (๚ ๛ ฯ).
/// * Newlines.
///
/// # Examples
/// ```
/// use agi_thainlp::sent_tokenize;
/// let sents = sent_tokenize("I love cats. Do you?");
/// assert_eq!(sents, vec!["I love cats.", "Do you?"]);
/// ```
pub fn sent_tokenize(text: &str) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    // The Rust regex crate doesn't support lookbehind, so we implement
    // sentence splitting manually.
    let mut sentences: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut start = 0;

    let is_upper_or_thai = |c: char| {
        c.is_uppercase() || ('\u{0e00}'..='\u{0e7f}').contains(&c)
    };
    let is_sent_end = |c: char| ".!?".contains(c);
    let is_thai_sent_end = |c: char| "๚๛ฯ".contains(c);

    let mut i = 0;
    while i < n {
        let ch = chars[i];
        if is_sent_end(ch)
            && i + 1 < n
            && chars[i + 1] == ' '
            && i + 2 < n
            && is_upper_or_thai(chars[i + 2])
        {
            // Split after the sentence-end punctuation
            let seg: String = chars[start..=i].iter().collect();
            let seg = seg.trim().to_string();
            if !seg.is_empty() {
                sentences.push(seg);
            }
            // Skip whitespace
            i += 2;
            while i < n && chars[i] == ' ' {
                i += 1;
            }
            start = i;
        } else if is_thai_sent_end(ch) {
            let seg: String = chars[start..=i].iter().collect();
            let seg = seg.trim().to_string();
            if !seg.is_empty() {
                sentences.push(seg);
            }
            i += 1;
            while i < n && (chars[i] == ' ' || chars[i] == '\n' || chars[i] == '\r') {
                i += 1;
            }
            start = i;
        } else if ch == '\n' || ch == '\r' {
            let seg: String = chars[start..i].iter().collect();
            let seg = seg.trim().to_string();
            if !seg.is_empty() {
                sentences.push(seg);
            }
            i += 1;
            while i < n && (chars[i] == '\n' || chars[i] == '\r') {
                i += 1;
            }
            start = i;
        } else {
            i += 1;
        }
    }
    // Tail
    if start < n {
        let seg: String = chars[start..].iter().collect();
        let seg = seg.trim().to_string();
        if !seg.is_empty() {
            sentences.push(seg);
        }
    }
    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcc_basic() {
        let result = tcc_tokenize("กา");
        assert_eq!(result, vec!["กา"]);
    }

    #[test]
    fn tcc_two_syllables() {
        let result = tcc_tokenize("กาแฟ");
        assert!(result.contains(&"กา".to_string()));
        assert!(result.contains(&"แฟ".to_string()));
    }

    #[test]
    fn tcc_empty() {
        assert!(tcc_tokenize("").is_empty());
    }

    #[test]
    fn tcc_reconstruct() {
        let text = "ภาษาไทย";
        let tokens = tcc_tokenize(text);
        assert_eq!(tokens.join(""), text);
    }

    #[test]
    fn tcc_non_thai() {
        let result = tcc_tokenize("abc");
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn syllable_tokenize_alias() {
        let text = "ประเทศ";
        assert_eq!(syllable_tokenize(text), tcc_tokenize(text));
    }

    #[test]
    fn sent_basic() {
        let result = sent_tokenize("I love cats. Do you?");
        assert_eq!(result, vec!["I love cats.", "Do you?"]);
    }

    #[test]
    fn sent_empty() {
        assert!(sent_tokenize("").is_empty());
    }

    #[test]
    fn sent_newline() {
        let result = sent_tokenize("line one\nline two");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn sent_single() {
        let result = sent_tokenize("วันนี้ฉันไปตลาด");
        assert_eq!(result, vec!["วันนี้ฉันไปตลาด"]);
    }
}

