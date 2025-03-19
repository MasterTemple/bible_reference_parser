use crate::passage::segment::types::chapter_verse::ChapterVerse;
use crate::Result;

use super::{book_data::BookData, book_segment::BookSegment};

pub struct ReferenceParser {
    book_data: BookData,
}

// impl ReferenceParser {
//     pub fn parse_single_book_chapter_verse<'a>(
//         &'a self,
//         input: &'_ str,
//     ) -> Result<BookSegment<'a, ChapterVerse>> {
//         Ok(self
//             .book_data
//             .parse_single_book_chapter_verse(input)
//             .ok_or_else(|| format!("Could not parse a single book/chapter/verse"))?)
//     }
// }
