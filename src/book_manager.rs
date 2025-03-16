use std::collections::BTreeMap;

use derive_more::{Deref, DerefMut, IntoIterator};
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{book_chapter_verse::BookChapterVerse, book_segment::BookSegment, parse::ParsableSegment, passage_segments::chapter_verse::ChapterVerse, segments::{BookPassageSegments, PassageSegments}};

/// eventually this will have a locale so i can group by languages
#[derive(Clone, Debug)]
pub struct BookManager {
    /// regex to match all book names or abbreviations that are part of this data set (for matching)
    pub book_regex: Regex,
    /// map of abbreviations and actual name (all lowercase) to book id (for searching)
    pub abbreviations_to_book_id: BTreeMap<String, u8>,
    /// map of book id to book name (for display)
    pub book_id_to_name: BTreeMap<u8, String>,
    /// map of book id to abbreviation (for display)
    pub book_id_to_abbreviation: BTreeMap<u8, String>,
}

static BOOKS_WITH_ABBREVIATIONS_JSON: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/books_with_abbreviations.json"));

/**
Example:
```jsonc
[
  {
    "id": 1,
    "book": "Genesis",
    "abbreviation": "Gn",
    "abbreviations": [
      "gen",
      "ge",
      "gn"
    ]
  },
  // ...
]
```
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookWithAbbreviations {
    /// - book id, starting at 1
    /// - Genesis = 1
    /// - Matthew = 40
    #[serde(alias = "num")]
    #[serde(alias = "number")]
    id: u8,

    /// - the display name
    /// - case is kept
    /// - does not need to be repeated in abbreviations
    #[serde(alias = "book")]
    #[serde(alias = "book_name")]
    #[serde(alias = "display_name")]
    name: String,

    /// - the display abbreviation
    /// - case is kept
    /// - does not need to be repeated in abbreviations
    /// - TODO: if not provided, the first abbreviations
    #[serde(alias = "abbr")]
    #[serde(alias = "abbrv")]
    #[serde(alias = "abbrev")]
    abbreviation: String,

    /// - does not need to be repeated in abbreviations
    /// - meant for matching/parsing references
    #[serde(alias = "abbrs")]
    #[serde(alias = "abbrvs")]
    #[serde(alias = "abbrevs")]
    #[serde(default)]
    abbreviations: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Deref, DerefMut, IntoIterator)]
pub struct BookWithAbbreviationsList(Vec<BookWithAbbreviations>);

impl Default for BookWithAbbreviationsList {
    fn default() -> Self {
        serde_json::from_str(&BOOKS_WITH_ABBREVIATIONS_JSON)
            .map_err(|_| format!("Could not parse default file")).unwrap()
    }
}

impl Default for BookManager {
    fn default() -> Self {
        let data = BookWithAbbreviationsList::default();
        Self::new(data).expect("The default provided data data should always compile")
    }
}

// #[derive(Clone, Debug)]
// pub struct BookInfo<'a> {
//     id: u8,
//     name: &'a str,
//     abbr: &'a str,
// }

impl<'a> BookManager {
    /// - You only want to use this when you have custom data
    /// - If you would like English book names, please just use [`Default::default()`]
    pub fn new(data: BookWithAbbreviationsList) -> Result<Self, String> {
        let mut abbreviations_to_book_id = BTreeMap::new();
        let mut book_id_to_name = BTreeMap::new();
        let mut book_id_to_abbreviation = BTreeMap::new();

        for book in data {
            abbreviations_to_book_id.insert(BookManager::normalize_book_name(&book.name), book.id);
            book_id_to_name.insert(book.id, book.name);
            book_id_to_abbreviation.insert(book.id, book.abbreviation);
            for abbreviation in book.abbreviations {
                abbreviations_to_book_id.insert(BookManager::normalize_book_name(&abbreviation), book.id);
            }
        }

        // keys are already unique
        let books_pattern: String = abbreviations_to_book_id.keys().join("|");
        // I added the period so that people can use it in abbreviations
        let book_regex = Regex::new(format!(r"\b(((?:)(?i){books_pattern})[A-z]*)\.?").as_str())
            .map_err(|e| format!("Failed to compile book_regex because of bad user input.\n{e}"))?;

        Ok(BookManager {
            book_regex,
            abbreviations_to_book_id,
            book_id_to_name,
            book_id_to_abbreviation,
        })
    }

    pub fn normalize_book_name(name: &str) -> String {
        name.to_lowercase().trim_end_matches(".").trim().to_string()
    }

    pub fn parse_single_book_chapter_verse(&'a self, input: &'_ str) -> Option<BookSegment<ChapterVerse>> {
        let book_match = self.book_regex.find_iter(input).next()?;
        let book_name = &Self::normalize_book_name(book_match.as_str());
        // (this should always match though)
        let book_id = *self.abbreviations_to_book_id.get(book_name)?;
        let chapter_verse = ChapterVerse::parse(&input[book_match.end()..]).ok()?;
        Some(*BookChapterVerse::new(book_id, chapter_verse.chapter, chapter_verse.verse).ok()?)
    }

    /// This is meant only to parse a single Bible verse
    /// but when do i only want single verses? when specifying the text, i think that is it
    pub fn parse_reference(&'a self, input: &'_ str) -> Option<BookPassageSegments> {
        let book_match = self.book_regex.find_iter(input).next()?;
        let book_name = &Self::normalize_book_name(book_match.as_str());
        // (this should always match though)
        let book_id = *self.abbreviations_to_book_id.get(book_name)?;

        // let book_display_name = self.book_id_to_name.get(&book_id)?;
        // let book_abbr_name = self.book_id_to_abbreviation.get(&book_id)?;
        // let book_info = BookInfo {
        //     id: book_id,
        //     name: &book_display_name,
        //     abbr: &book_abbr_name,
        // };

        let segments = PassageSegments::parse(&input[book_match.end()..]).ok()?;
        Some(segments.with_book(book_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_manager() {
        let manager = BookManager::default();
        assert!(manager.parse_reference("John 1:1").is_some());
        assert!(manager.parse_reference("John1:1").is_some());
        assert!(manager.parse_reference(" John 1:1 ").is_some());
        assert!(manager.parse_reference(" Jn 1:1 ").is_some());
        assert!(manager.parse_reference(" Jn. 1:1 ").is_some());
        assert!(manager.parse_reference(" Jn.1:1 ").is_some());
        assert!(manager.parse_reference(" Jn.1.1 ").is_some());
        assert!(manager.parse_reference(" Jn 1.1 ").is_some());
        assert!(manager.parse_reference(" Jn1.1 ").is_some());

        assert!(manager.parse_reference(" Jn1.1-2").unwrap().segments.len() == 1);
        assert!(manager.parse_reference(" Jn1.1,2").unwrap().segments.len() == 2);

        assert!(dbg!(manager.parse_reference("Jn1.1")).is_some());
    }
}
