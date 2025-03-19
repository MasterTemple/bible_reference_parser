use crate::{
    books::{
        book_chapter_verse_id::BookChapterVerse, book_segment::BookSegment,
        reference_parser::ReferenceParser,
    },
    passage::segment::types::chapter_verse::ChapterVerse,
};

use crate::Result;

pub trait ParsableReference {
    fn parse_reference<'a>(
        &self,
        parser: &'a ReferenceParser,
    ) -> Result<BookSegment<'a, ChapterVerse>>;
}

// impl ParsableReference for &str {
//     fn parse_reference<'a>(
//         &self,
//         parser: &'a ReferenceParser,
//     ) -> Result<BookSegment<'a, ChapterVerse>> {
//         // if let Ok(bcv) = BookChapterVerse::from_id_string(self) {
//         //     return bcv.
//         // }
//         parser.parse_single_book_chapter_verse(self)
//     }
// }
//
// impl ParsableReference for u16 {
//     fn parse_reference<'a>(
//         &self,
//         parser: &'a ReferenceParser,
//     ) -> Result<BookSegment<'a, ChapterVerse>> {
//         BookChapterVerse::from_verse(*self)
//     }
// }
