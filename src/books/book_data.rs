use std::collections::BTreeMap;

use itertools::Itertools;
use regex::Regex;
use serde::Serialize;

use crate::passage::{segment::{individual_parse::ParsableSegment, types::chapter_verse::ChapterVerse}, segments::PassageSegments};

use super::{book_segment::BookSegment, book_segments::BookPassageSegments, data::book_with_abbreviations::BookWithAbbreviationsList};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BookInfo<'a> {
    pub id: u8,
    pub name: &'a str,
    // name: String,
    pub abbr: &'a str,
}

#[derive(Clone, Debug)]
pub struct BookData {
    /// regex to match all book names or abbreviations that are part of this data set (for matching)
    pub book_regex: Regex,
    /// map of abbreviations and actual name (all lowercase) to book id (for searching)
    pub abbreviations_to_book_id: BTreeMap<String, u8>,
    /// map of book id to book name (for display)
    pub book_id_to_name: BTreeMap<u8, String>,
    /// map of book id to abbreviation (for display)
    pub book_id_to_abbreviation: BTreeMap<u8, String>,
}

impl<'a> BookData {
    /// - You only want to use this when you have custom data
    /// - If you would like English book names, please just use [`Default::default()`]
    pub fn new(data: BookWithAbbreviationsList) -> Result<Self, String> {
        let mut abbreviations_to_book_id = BTreeMap::new();
        let mut book_id_to_name = BTreeMap::new();
        let mut book_id_to_abbreviation = BTreeMap::new();

        for book in data {
            abbreviations_to_book_id.insert(BookData::normalize_book_name(&book.name), book.id);
            book_id_to_name.insert(book.id, book.name);
            book_id_to_abbreviation.insert(book.id, book.abbreviation);
            for abbreviation in book.abbreviations {
                abbreviations_to_book_id.insert(BookData::normalize_book_name(&abbreviation), book.id);
            }
        }

        // keys are already unique
        let books_pattern: String = abbreviations_to_book_id.keys().join("|");
        // I added the period so that people can use it in abbreviations
        let book_regex = Regex::new(format!(r"\b(((?:)(?i){books_pattern})[A-z]*)\.?").as_str())
            .map_err(|e| format!("Failed to compile book_regex because of bad user input.\n{e}"))?;

        Ok(BookData {
            book_regex,
            abbreviations_to_book_id,
            book_id_to_name,
            book_id_to_abbreviation,
        })
    }

    pub fn normalize_book_name(name: &str) -> String {
        name.to_lowercase().trim_end_matches(".").trim().to_string()
    }

    pub fn parse_single_book_chapter_verse(&'a self, input: &'_ str) -> Option<BookSegment<'a, ChapterVerse>> {
        let book_match = self.book_regex.find_iter(input).next()?;
        let book_name = &Self::normalize_book_name(book_match.as_str());
        // (this should always match though)
        let book_id = *self.abbreviations_to_book_id.get(book_name)?;
        let chapter_verse = ChapterVerse::parse(&input[book_match.end()..]).ok()?;
        todo!()
        // Some(*BookChapterVerse::new(book_id, chapter_verse.chapter, chapter_verse.verse).ok()?)
    }

    /// This is meant only to parse a single Bible verse
    /// but when do i only want single verses? when specifying the text, i think that is it
    pub fn parse_reference(&'a self, input: &'_ str) -> Option<BookPassageSegments<'a>> {
        let book_match = self.book_regex.find_iter(input).next()?;
        let book_name = &Self::normalize_book_name(book_match.as_str());
        // (this should always match though)
        let book_id = *self.abbreviations_to_book_id.get(book_name)?;

        let book_display_name = self.book_id_to_name.get(&book_id)?;
        let book_abbr_name = self.book_id_to_abbreviation.get(&book_id)?;
        let book_info = BookInfo {
            id: book_id,
            name: &book_display_name,
            abbr: &book_abbr_name,
        };

        let segments = PassageSegments::parse(&input[book_match.end()..]).ok()?;
        Some(segments.with_book(book_info))
    }
}

impl Default for BookData {
    fn default() -> Self {
        let data = BookWithAbbreviationsList::default();
        Self::new(data).expect("The default provided data data should always compile")
    }
}
