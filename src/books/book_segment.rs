use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::passage::segment::{
    any_segment::AnySegment,
    segment::SegmentFns,
    types::{
        chapter_range::ChapterRange, chapter_verse::ChapterVerse,
        chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter,
        full_chapter_range::FullChapterRange,
    },
};

use super::book_data::BookInfo;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BookSegment<'a, Segment: SegmentFns> {
    pub book: BookInfo<'a>,
    pub segment: Segment,
}

impl<'a> BookSegment<'a, ChapterVerse> {
    pub fn chapter_verse(
        book: BookInfo<'a>,
        chapter: u8,
        verse: u8,
    ) -> BookSegment<'a, ChapterVerse> {
        BookSegment::new(book, ChapterVerse::new(chapter, verse))
    }
}

impl<'a> BookSegment<'a, ChapterVerseRange> {
    pub fn chapter_verse_range(
        book: BookInfo<'a>,
        chapter: u8,
        start_verse: u8,
        end_verse: u8,
    ) -> BookSegment<'a, ChapterVerseRange> {
        BookSegment::new(
            book,
            ChapterVerseRange::new(chapter, start_verse, end_verse),
        )
    }
}

impl<'a> BookSegment<'a, ChapterRange> {
    pub fn chapter_range(
        book: BookInfo<'a>,
        start_chapter: u8,
        start_verse: u8,
        end_chapter: u8,
        end_verse: u8,
    ) -> BookSegment<'a, ChapterRange> {
        BookSegment::new(
            book,
            ChapterRange::new(start_chapter, start_verse, end_chapter, end_verse),
        )
    }
}

impl<'a> BookSegment<'a, FullChapter> {
    pub fn full_chapter(book: BookInfo<'a>, chapter: u8) -> BookSegment<'a, FullChapter> {
        BookSegment::new(book, FullChapter::new(chapter))
    }
}

impl<'a> BookSegment<'a, FullChapterRange> {
    pub fn full_chapter_range(
        book: BookInfo<'a>,
        start: u8,
        end: u8,
    ) -> BookSegment<'a, FullChapterRange> {
        BookSegment::new(book, FullChapterRange::new(start, end))
    }
}

impl<'a, Segment: SegmentFns> BookSegment<'a, Segment> {
    pub fn new(book: BookInfo<'a>, segment: Segment) -> BookSegment<'a, Segment> {
        Self { book, segment }
    }

    pub fn generalize(self) -> BookSegment<'a, AnySegment> {
        BookSegment {
            book: self.book,
            segment: self.segment.into(),
        }
    }
}
