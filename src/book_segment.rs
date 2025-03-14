use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::{compare::SegmentCompare, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}, segment::PassageSegment};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookSegment<Segment: SegmentCompare> {
    pub book: u8,
    pub segment: Segment,
}

impl BookSegment<ChapterVerse> {
    pub fn chapter_verse(book: u8, chapter: u8, verse: u8) -> BookSegment<ChapterVerse> {
        BookSegment::new(book, ChapterVerse::new(chapter, verse))
    }
}

impl BookSegment<ChapterVerseRange> {
    pub fn chapter_verse_range(book: u8, chapter: u8, start_verse: u8, end_verse: u8) -> BookSegment<ChapterVerseRange> {
        BookSegment::new(book, ChapterVerseRange::new(chapter, start_verse, end_verse))
    }
}

impl BookSegment<ChapterRange> {
    pub fn chapter_range(book: u8, start_chapter: u8, start_verse: u8, end_chapter: u8, end_verse: u8) -> BookSegment<ChapterRange> {
        BookSegment::new(book, ChapterRange::new(start_chapter, start_verse, end_chapter, end_verse))
    }
}

impl BookSegment<FullChapter> {
    pub fn full_chapter(book: u8, chapter: u8) -> BookSegment<FullChapter> {
        BookSegment::new(book, FullChapter::new(chapter))
    }
}

impl BookSegment<FullChapterRange> {
    pub fn full_chapter_range(book: u8, start: u8, end: u8) -> BookSegment<FullChapterRange> {
        BookSegment::new(book, FullChapterRange::new(start, end))
    }
}

impl<Segment: SegmentCompare> BookSegment<Segment> {
    pub fn new(book: u8, segment: Segment) -> Self {
        Self {
            book,
            segment,
        }
    }

    pub fn generalize(self) -> BookSegment<PassageSegment> {
        BookSegment {
            book: self.book,
            segment: self.segment.into(),
        }
    }
}
