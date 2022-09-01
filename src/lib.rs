use clap::ValueEnum;
use std::collections::HashMap;

pub struct Analyzer {
    map: HashMap<CharType, HashMap<char, u32>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(CharType::Hiragana, HashMap::new());
        map.insert(CharType::Katakana, HashMap::new());
        map.insert(CharType::Kanji, HashMap::new());
        map.insert(CharType::Other, HashMap::new());
        Self { map }
    }
}

impl Analyzer {
    pub fn read_str(&mut self, str: &String) {
        for char in str.chars() {
            self.read_char(char);
        }
    }

    pub fn read_char(&mut self, char: char) {
        let char_type = get_char_type(char);
        let new_count = if let Some(count) = self.map[&char_type].get(&char) {
            count + 1
        } else {
            1
        };
        self.map
            .get_mut(&char_type)
            .unwrap()
            .insert(char, new_count);
    }

    pub fn char_freqs(&self, char_type: &CharType) -> Vec<(&char, &u32)> {
        let mut freq_vec = self.map[char_type].iter().collect::<Vec<_>>();
        freq_vec.sort_by(|(_, c1), (_, c2)| c1.cmp(c2));
        freq_vec
    }

    pub fn get_unique_count(&self, char_type: &CharType) -> u32 {
        self.map[char_type].len() as u32
    }

    pub fn get_total_count(&self, char_type: &CharType) -> u32 {
        let mut total_count = 0;
        for (_, count) in &self.map[&char_type] {
            total_count += count;
        }
        total_count
    }
}

#[derive(Debug, PartialEq, Eq, Hash, ValueEnum, Clone, Copy)]
pub enum CharType {
    Hiragana,
    Katakana,
    Kanji,
    Other,
}

impl std::fmt::Display for CharType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharType::Hiragana => write!(f, "Hiragana"),
            CharType::Katakana => write!(f, "Katakana"),
            CharType::Kanji => write!(f, "Kanji"),
            CharType::Other => write!(f, "Other"),
        }
    }
}

pub const ALL_CHAR_TYPES: &[CharType] = &[
    CharType::Hiragana,
    CharType::Katakana,
    CharType::Kanji,
    CharType::Other,
];

fn get_char_type(char: char) -> CharType {
    match char as u32 {
        0x3041..=0x3096 => CharType::Hiragana,
        0x30a1..=0x30fa => CharType::Katakana,
        // Note that this also includes chinese characters see https://en.wikipedia.org/wiki/CJK_Unified_Ideographs
        // Probably not nesscery to include all the extensions but lets just be inclusive
        0x4e00..=0x9fff
        | 0x3400..=0x4dbf
        | 0x2a700..=0x2b73f
        | 0x2b740..=0x2b81f
        | 0x2b820..=0x2ceaf
        | 0x2ceb0..=0x2ebef
        | 0xf900..=0xfaff => CharType::Kanji,
        _ => CharType::Other,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Analyzer, CharType};

    #[test]
    fn correct_count() {
        let mut parser = Analyzer::default();
        let test_str =
            "だから今日も一旦家に帰って、ランドセルを置いてからすぐに習い事へ向かう用意をする。でも昨日、";
        parser.read_str(&String::from(test_str));
        assert_eq!(parser.get_total_count(&CharType::Hiragana), 24);
        assert_eq!(parser.get_total_count(&CharType::Katakana), 5);
        assert_eq!(parser.get_total_count(&CharType::Kanji), 14);
        assert_eq!(parser.get_total_count(&CharType::Other), 3);
        assert_eq!(parser.get_unique_count(&CharType::Hiragana), 15);
        assert_eq!(parser.get_unique_count(&CharType::Katakana), 5);
        assert_eq!(parser.get_unique_count(&CharType::Kanji), 13);
        assert_eq!(parser.get_unique_count(&CharType::Other), 2);
    }
}
