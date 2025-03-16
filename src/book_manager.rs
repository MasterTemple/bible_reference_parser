use std::collections::BTreeMap;

use regex::Regex;

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

#[derive(Clone, Debug)]
pub struct BookInfo<'a> {
    id: u8,
    name: &'a str,
    abbr: &'a str,
}

impl<'a> BookManager {
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

        let book_display_name = self.book_id_to_name.get(&book_id)?;
        let book_abbr_name = self.book_id_to_abbreviation.get(&book_id)?;
        let book_info = BookInfo {
            id: book_id,
            name: &book_display_name,
            abbr: &book_abbr_name,
        };

        let segments = PassageSegments::parse(&input[book_match.end()..]).ok()?;
        Some(segments.with_book(book_id))
        // Some(BookPassageSegments {
        //     // book: book_info,
        //     book: book_id,
        //     segments,
        // })
    }

    // pub fn parse_file(path: &Path) -> AnyResult<Self> {
    //     BookManagerCreator::try_formats(path)?.try_into()
    // }
}
