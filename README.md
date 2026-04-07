# AGI-ThaiNLP
Rewriting all pythainlp for new generation

A from-scratch reimplementation of Thai NLP functions in **Python** and **Rust**,
without relying on PyThaiNLP's bundled resources (dictionaries, models, etc.).

## Modules

| Module | Description |
|--------|-------------|
| `core` | Thai character constants (consonants, vowels, tonemarks, digits, …) |
| `tokenize.tcc` | Thai Character Cluster (TCC) tokenizer |
| `tokenize.newmm` | **NewMM** dictionary-based word tokenizer (Maximum Matching + Viterbi DP) |
| `tokenize.sent` | Rule-based sentence tokenizer |
| `transliterate.rtgs` | RTGS romanization (Royal Thai General System) |
| `soundex.udom83` | Udom83 soundex (7-char code) |
| `soundex.lk82` | LK82 soundex (5-char code) |
| `util.collate` | Thai dictionary-order sorting |
| `util.digits` | Thai digit ↔ Arabic digit conversion; Thai number words |
| `util.normalize` | Remove tonemarks |

## Python package

```bash
cd python
pip install -e .
```

```python
from agithainlp import tcc_tokenize, romanize, udom83, lk82, collate, num_to_thaiword, word_tokenize

print(tcc_tokenize("กาแฟ"))       # ['กา', 'แฟ']
print(word_tokenize("ผมชอบกินข้าว", keep_whitespace=False))  # ['ผม', 'ชอบ', 'กิน', 'ข้าว']
print(romanize("กาแฟ"))           # 'kafae'
print(udom83("รัก"))              # 'ร100000'
print(lk82("รัก"))               # 'ร1000'
print(collate(["ไก่", "เกิด", "กาล"]))  # ['กาล', 'เกิด', 'ไก่']
print(num_to_thaiword(21))        # 'ยี่สิบเอ็ด'
```

Run tests:
```bash
cd python && python -m pytest tests/ -v
```

## Rust crate

```bash
cd rust
cargo test
cargo run
```

```rust
use agi_thainlp::*;

fn main() {
    println!("{:?}", tcc_tokenize("กาแฟ"));   // ["กา", "แฟ"]
    println!("{:?}", word_tokenize("ผมชอบกินข้าว", None, false)); // ["ผม", "ชอบ", "กิน", "ข้าว"]
    println!("{}", romanize("กาแฟ"));          // kafae
    println!("{}", udom83("รัก"));             // ร100000
    println!("{}", num_to_thaiword(21));       // ยี่สิบเอ็ด
}
```

## No external resources

All algorithms are implemented from scratch:
- **TCC tokenization**: regex-based pattern matching (Theeramunkong et al. 2000)
- **RTGS romanization**: rule-based consonant/vowel substitution tables
- **Soundex**: phonetic encoding rules (Udom83 / LK82 papers)
- **Collation**: sort-key normalization for Thai dictionary ordering
- **Number conversion**: arithmetic decomposition into Thai word components

