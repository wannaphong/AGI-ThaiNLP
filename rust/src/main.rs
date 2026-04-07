use agi_thainlp::*;

fn main() {
    // Demo: show basic functionality
    let consonants: Vec<char> = THAI_CONSONANTS.chars().collect();
    println!("Thai consonants ({} total): {}", consonants.len(), THAI_CONSONANTS);

    let text = "กาแฟ";
    let tcc_tokens = tcc_tokenize(text);
    println!("TCC tokens for \"{text}\": {tcc_tokens:?}");

    let roman = romanize(text);
    println!("Romanize \"{text}\": {roman}");

    let code83 = udom83("รัก");
    println!("Udom83 soundex of \"รัก\": {code83}");

    let code82 = lk82("รัก");
    println!("LK82 soundex of \"รัก\": {code82}");

    println!("num_to_thaiword(21) = {}", num_to_thaiword(21));
    println!("thai_digit_to_arabic_digit(\"๑๒๓\") = {}", thai_digit_to_arabic_digit("๑๒๓"));

    let words = word_tokenize("ผมชอบกินข้าวผัด", None, false);
    println!("NewMM word_tokenize(\"ผมชอบกินข้าวผัด\"): {words:?}");

    let mixed = word_tokenize("I love ข้าวผัด", None, true);
    println!("NewMM word_tokenize(\"I love ข้าวผัด\"): {mixed:?}");
}
