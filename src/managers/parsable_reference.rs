use crate::{books::{book_segment::BookSegment, reference_parser::ReferenceParser}, passage::segment::types::chapter_verse::ChapterVerse};

use crate::Result;

pub trait ParsableReference {
    fn parse_reference<'a>(&self, parser: &'a ReferenceParser) -> Result<BookSegment<'a, ChapterVerse>>;
}
