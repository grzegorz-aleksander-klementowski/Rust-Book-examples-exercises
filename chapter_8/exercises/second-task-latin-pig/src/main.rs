// Convert strings to pig latin. The first consonant of
// each word is moved to the end of the word and ay is added,
// so first becomes irst-fay. Words that start with a
// vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

use std::str::Bytes;

fn pig_latin(word: &str) -> String {
    let first_letter: Option<char> = word.chars().next();

    let is_vowel: bool = matches!(
        first_letter,
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u')
    );

    let mut word_to_change = String::from(word);
    if is_vowel {
        word_to_change.push_str("hay");
        word_to_change
    } else {
        let n_utf_b = word_to_change.chars().next().unwrap().len_utf8();
        word_to_change.drain(..n_utf_b);
        word_to_change.push_str("fay");
        word_to_change
    }
}

fn main() {
    println!("The programming is writen in tests. Write `cargo test`.");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_piglatin_consonant_word() {
        let result = pig_latin("first");
        assert_eq!("irstfay", result)
    }

    #[test]
    fn test_piglatin_vowel_word() {
        let result = pig_latin("apple");
        assert_eq!("applehay", result)
    }
}
