use clap::ValueEnum;
use std::collections::HashMap;

pub struct Analyzer {
    map: HashMap<CharKind, HashMap<char, u32>>,
}

impl Default for Analyzer {
    fn default() -> Self {
        let mut map = HashMap::new();
        for kind in ALL_CHAR_KINDS {
            map.insert(*kind, HashMap::new());
        }
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
        let kind = get_char_kind(char);
        let new_count = if let Some(count) = self.map[&kind].get(&char) {
            count + 1
        } else {
            1
        };
        self.map.get_mut(&kind).unwrap().insert(char, new_count);
    }

    pub fn char_freqs(&self, kind: CharKind) -> Vec<(&char, &u32)> {
        let mut freq_vec = self.map[&kind].iter().collect::<Vec<_>>();
        freq_vec.sort_by(|(_, c1), (_, c2)| c1.cmp(c2));
        freq_vec
    }

    pub fn get_unique_count(&self, kind: CharKind) -> u32 {
        self.map[&kind].len() as u32
    }

    pub fn get_total_count(&self, kind: CharKind) -> u32 {
        let mut total_count = 0;
        for (_, count) in &self.map[&kind] {
            total_count += count;
        }
        total_count
    }
}

#[derive(Debug, PartialEq, Eq, Hash, ValueEnum, Clone, Copy)]
pub enum CharKind {
    Hiragana,
    Katakana,
    Kanji,
    Other,
}

impl std::fmt::Display for CharKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharKind::Hiragana => write!(f, "Hiragana"),
            CharKind::Katakana => write!(f, "Katakana"),
            CharKind::Kanji => write!(f, "Kanji"),
            CharKind::Other => write!(f, "Other"),
        }
    }
}

pub const ALL_CHAR_KINDS: &[CharKind] = &[
    CharKind::Hiragana,
    CharKind::Katakana,
    CharKind::Kanji,
    CharKind::Other,
];

fn get_char_kind(char: char) -> CharKind {
    match char as u32 {
        0x3041..=0x3096 => CharKind::Hiragana,
        0x30a1..=0x30fa => CharKind::Katakana,
        // Note that this also includes chinese characters see https://en.wikipedia.org/wiki/CJK_Unified_Ideographs
        // Probably not nesscery to include all the extensions but lets just be inclusive
        0x4e00..=0x9fff
        | 0x3400..=0x4dbf
        | 0x2a700..=0x2b73f
        | 0x2b740..=0x2b81f
        | 0x2b820..=0x2ceaf
        | 0x2ceb0..=0x2ebef
        | 0xf900..=0xfaff => CharKind::Kanji,
        _ => CharKind::Other,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Analyzer, CharKind};

    #[test]
    fn correct_count() {
        let mut parser = Analyzer::default();
        let test_str =
            "だから今日も一旦家に帰って、ランドセルを置いてからすぐに習い事へ向かう用意をする。でも昨日、";
        parser.read_str(&String::from(test_str));
        assert_eq!(parser.get_total_count(CharKind::Hiragana), 24);
        assert_eq!(parser.get_total_count(CharKind::Katakana), 5);
        assert_eq!(parser.get_total_count(CharKind::Kanji), 14);
        assert_eq!(parser.get_total_count(CharKind::Other), 3);
        assert_eq!(parser.get_unique_count(CharKind::Hiragana), 15);
        assert_eq!(parser.get_unique_count(CharKind::Katakana), 5);
        assert_eq!(parser.get_unique_count(CharKind::Kanji), 13);
        assert_eq!(parser.get_unique_count(CharKind::Other), 2);
    }
}
