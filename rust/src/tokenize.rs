//! Thai tokenization: TCC (Thai Character Cluster), sentence splitting,
//! and NewMM (New Maximum Matching) word tokenization.
//!
//! The TCC algorithm is based on:
//!   Theeramunkong, T. et al. (2000). Character cluster based Thai
//!   information retrieval. IRAL 2000.
//!
//! NewMM is based on:
//!   Haruechaiyasak, C. et al. (2009). A Comparative Study on Thai Word
//!   Segmentation Approaches.

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use crate::words::THAI_WORDS;

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

// ---------------------------------------------------------------------------
// NewMM (New Maximum Matching) word tokenizer
// ---------------------------------------------------------------------------

/// A prefix trie for O(k) word lookup (k = word length).
struct Trie {
    /// children[ch] holds the next node for character ch
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    fn add(&mut self, word: &str) {
        let mut node = self;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(Trie::new);
        }
        node.is_end = true;
    }

    /// Return all words in the trie that are prefixes of `text` (shortest first).
    fn prefixes<'a>(&self, text: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        let mut node = self;
        let mut byte_end = 0;
        for ch in text.chars() {
            match node.children.get(&ch) {
                None => break,
                Some(next) => {
                    byte_end += ch.len_utf8();
                    node = next;
                    if node.is_end {
                        results.push(&text[..byte_end]);
                    }
                }
            }
        }
        results
    }
}

/// Build a Trie from a slice of words.
fn build_trie(words: &[&str]) -> Trie {
    let mut trie = Trie::new();
    for &w in words {
        trie.add(w);
    }
    trie
}

static DEFAULT_TRIE: Lazy<Trie> = Lazy::new(|| build_trie(THAI_WORDS));

/// Segment a single Thai text chunk using the NewMM DP algorithm.
fn newmm_segment(text: &str, trie: &Trie) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    if n == 0 {
        return Vec::new();
    }

    // Compute byte start of each character position
    let mut byte_starts: Vec<usize> = Vec::with_capacity(n + 1);
    let mut offset = 0;
    for &c in &chars {
        byte_starts.push(offset);
        offset += c.len_utf8();
    }
    byte_starts.push(offset); // sentinel for n

    let inf = n + 1;
    let mut dp = vec![inf; n + 1];
    dp[0] = 0;
    let mut prev: Vec<usize> = vec![0usize; n + 1];

    for i in 0..n {
        if dp[i] == inf {
            continue;
        }
        let cost = dp[i] + 1;
        let slice = &text[byte_starts[i]..];

        // 1. Dictionary words starting at char position i
        for word in trie.prefixes(slice) {
            let word_len = word.chars().count();
            let j = i + word_len;
            if cost < dp[j] {
                dp[j] = cost;
                prev[j] = i;
            }
        }

        // 2. TCC fallback — always offer one TCC unit
        let tcc_units = tcc_tokenize(slice);
        let unit_len = tcc_units
            .first()
            .map(|u| u.chars().count())
            .unwrap_or(1);
        let j = i + unit_len;
        if j <= n && cost < dp[j] {
            dp[j] = cost;
            prev[j] = i;
        }
    }

    // Reconstruct
    if dp[n] == inf {
        return vec![text.to_string()];
    }
    let mut tokens: Vec<String> = Vec::new();
    let mut pos = n;
    while pos > 0 {
        let start = prev[pos];
        tokens.push(text[byte_starts[start]..byte_starts[pos]].to_string());
        pos = start;
    }
    tokens.reverse();
    tokens
}

/// Tokenize Thai text into words using the NewMM (Maximum Matching) algorithm.
///
/// NewMM uses a dictionary and dynamic programming to find the segmentation
/// that minimises the total number of tokens (equivalent to maximum-length
/// matching).  Unknown Thai sub-strings are split at TCC boundaries.
///
/// # Arguments
/// * `text` — text to tokenize (may contain Thai, ASCII, and whitespace)
/// * `custom_dict` — optional slice of words to use instead of the built-in
///   word list.  Pass `None` to use the built-in dictionary.
/// * `keep_whitespace` — when `true`, whitespace tokens are included in
///   the result
///
/// # Examples
/// ```
/// use agi_thainlp::word_tokenize;
/// let tokens = word_tokenize("ผมชอบกินข้าว", None, false);
/// assert!(tokens.contains(&"ผม".to_string()));
/// assert!(tokens.join("") == "ผมชอบกินข้าว");
/// ```
pub fn word_tokenize(
    text: &str,
    custom_dict: Option<&[&str]>,
    keep_whitespace: bool,
) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }

    // Build or borrow the trie
    let custom_trie: Option<Trie> = custom_dict.map(build_trie);
    let trie: &Trie = match &custom_trie {
        Some(t) => t,
        None => &DEFAULT_TRIE,
    };

    let mut result: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        let ch = chars[i];
        if ch == ' ' || ch == '\t' {
            // Whitespace run
            let start = i;
            while i < n && (chars[i] == ' ' || chars[i] == '\t') {
                i += 1;
            }
            if keep_whitespace {
                result.push(chars[start..i].iter().collect());
            }
        } else if ch == '\n' || ch == '\r' {
            if keep_whitespace {
                result.push(ch.to_string());
            }
            i += 1;
        } else if '\u{0e00}' <= ch && ch <= '\u{0e7f}' {
            // Thai run
            let start = i;
            while i < n && '\u{0e00}' <= chars[i] && chars[i] <= '\u{0e7f}' {
                i += 1;
            }
            // Reconstruct byte slice for the Thai run
            let thai_run: String = chars[start..i].iter().collect();
            result.extend(newmm_segment(&thai_run, trie));
        } else {
            // Non-Thai, non-whitespace run
            let start = i;
            while i < n
                && chars[i] != ' '
                && chars[i] != '\t'
                && chars[i] != '\n'
                && chars[i] != '\r'
                && !(('\u{0e00}' <= chars[i]) && (chars[i] <= '\u{0e7f}'))
            {
                i += 1;
            }
            result.push(chars[start..i].iter().collect());
        }
    }
    result
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

    // --- NewMM tests ---

    #[test]
    fn word_tokenize_empty() {
        assert!(word_tokenize("", None, false).is_empty());
    }

    #[test]
    fn word_tokenize_reconstruct() {
        let text = "ผมชอบกินข้าว";
        let tokens = word_tokenize(text, None, false);
        assert_eq!(tokens.join(""), text);
    }

    #[test]
    fn word_tokenize_known_words() {
        let tokens = word_tokenize("วันนี้ดีมาก", None, false);
        assert!(tokens.contains(&"วันนี้".to_string()));
        // 'ดีมาก' may be one token (in dict) or two ('ดี' + 'มาก')
        assert!(
            tokens.iter().any(|t| t == "ดีมาก" || t == "ดี"),
            "expected ดีมาก or ดี in {tokens:?}"
        );
        assert_eq!(tokens.join(""), "วันนี้ดีมาก");
    }

    #[test]
    fn word_tokenize_custom_dict() {
        let dict = &["ประเทศไทย", "สวยงาม"];
        let tokens = word_tokenize("ประเทศไทยสวยงาม", Some(dict), false);
        assert!(tokens.contains(&"ประเทศไทย".to_string()));
        assert!(tokens.contains(&"สวยงาม".to_string()));
    }

    #[test]
    fn word_tokenize_whitespace_preserved() {
        let tokens = word_tokenize("I love กาแฟ", None, true);
        assert!(tokens.contains(&" ".to_string()));
    }

    #[test]
    fn word_tokenize_whitespace_dropped() {
        let tokens = word_tokenize("I love กาแฟ", None, false);
        assert!(!tokens.contains(&" ".to_string()));
    }

    #[test]
    fn word_tokenize_mixed() {
        let tokens = word_tokenize("I love ข้าวผัด", None, false);
        assert!(tokens.contains(&"I".to_string()));
        assert!(tokens.contains(&"love".to_string()));
    }

    #[test]
    fn word_tokenize_tcc_fallback() {
        // Empty custom dict → TCC fallback
        let tokens = word_tokenize("กาแฟ", Some(&[]), false);
        assert!(!tokens.is_empty());
        assert_eq!(tokens.join(""), "กาแฟ");
        assert!(tokens.len() <= 2); // at most 2 TCC units
    }
}

