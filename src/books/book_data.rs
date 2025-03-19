use std::collections::BTreeMap;

use crate::{passage::parse_segments::POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS, Result};
use itertools::Itertools;
use regex::{Match, Regex};
use serde::Serialize;

use crate::passage::{
    segment::{individual_parse::ParsableSegment, types::chapter_verse::ChapterVerse},
    segments::PassageSegments,
};

use super::{
    book_segment::BookSegment, book_segments::BookPassageSegments,
    data::book_with_abbreviations::BookWithAbbreviationsList,
};

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

impl BookData {
    /// - You only want to use this when you have custom data
    /// - If you would like English book names, please just use [`Default::default()`]
    pub fn new(data: BookWithAbbreviationsList) -> Result<Self> {
        let mut abbreviations_to_book_id = BTreeMap::new();
        let mut book_id_to_name = BTreeMap::new();
        let mut book_id_to_abbreviation = BTreeMap::new();

        for book in data {
            abbreviations_to_book_id.insert(BookData::normalize_book_name(&book.name), book.id);
            book_id_to_name.insert(book.id, book.name);
            book_id_to_abbreviation.insert(book.id, book.abbreviation);
            for abbreviation in book.abbreviations {
                abbreviations_to_book_id
                    .insert(BookData::normalize_book_name(&abbreviation), book.id);
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

    fn normalize_book_name(name: &str) -> String {
        name.to_lowercase().trim_end_matches(".").trim().to_string()
    }

    pub fn get_book_info<'a>(&'a self, book_id: u8) -> Option<BookInfo<'a>> {
        let book_display_name = self.book_id_to_name.get(&book_id)?;
        let book_abbr_name = self.book_id_to_abbreviation.get(&book_id)?;
        let book_info = BookInfo {
            id: book_id,
            name: &book_display_name,
            abbr: &book_abbr_name,
        };
        Some(book_info)
    }

    pub fn parse_book_info<'a>(&'a self, book: &'a str) -> Option<BookInfo<'a>> {
        let book_name = &Self::normalize_book_name(book);
        let book_id = *self.abbreviations_to_book_id.get(book_name)?;
        self.get_book_info(book_id)
    }

    pub fn iter_book_matches<'a>(
        &'a self,
        input: &'a str,
    ) -> Option<impl Iterator<Item = BookMatch<'a>> + 'a> {
        Some(
            self.book_regex
                .find_iter(input)
                .filter_map(move |book_match| {
                    let book_info = self.parse_book_info(book_match.as_str())?;
                    let remaining = &input[book_match.end()..];
                    let reference_match = POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS
                        .find_iter(remaining)
                        .next()?;

                    Some(BookMatch {
                        book_info,
                        book_match,
                        reference_match,
                    })
                }),
        )
    }

    pub fn find_book_match<'a>(&'a self, input: &'a str) -> Option<BookMatch<'a>> {
        let book_match = self.book_regex.find_iter(input).next()?;
        let book_info = self.parse_book_info(book_match.as_str())?;
        let reference_match = POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS
            .find_iter(&input[book_match.end()..])
            .next()?;

        Some(BookMatch {
            book_info,
            book_match,
            reference_match,
        })
    }
}

#[derive(Clone, Debug)]
pub struct BookMatch<'a> {
    pub book_info: BookInfo<'a>,
    pub book_match: Match<'a>,
    pub reference_match: Match<'a>,
}

impl<'a> BookMatch<'a> {
    pub fn parse(&self) -> Result<BookPassageSegments<'a>> {
        let segments = PassageSegments::parse(self.reference_match.as_str())?;
        Ok(segments.with_book(self.book_info))
    }
}

impl Default for BookData {
    fn default() -> Self {
        let data = BookWithAbbreviationsList::default();
        Self::new(data).expect("The default provided data data should always compile")
    }
}

#[cfg(test)]
mod tests {
    use super::BookData;

    #[test]
    fn book_match() {
        let bd = BookData::default();
        dbg!(bd.find_book_match("Genesis 1:1").unwrap().parse().unwrap());
        dbg!(bd.find_book_match("Gen.xiv.1").unwrap().parse().unwrap());
    }
}
