use serde::Serialize;

use crate::passage::{
    segment::{any_segment::AnySegment, segment::SegmentFns},
    segments::PassageSegments,
};

use super::{book_data::BookInfo, book_segment::BookSegment};

#[derive(Clone, Debug, Serialize)]
pub struct BookPassageSegments<'a> {
    pub book: BookInfo<'a>,
    pub segments: PassageSegments,
}

#[derive(Clone, Debug)]
pub struct BookPassageSegmentsIter<'a> {
    book: BookInfo<'a>,
    iter: <PassageSegments as IntoIterator>::IntoIter,
}

impl<'a> Iterator for BookPassageSegmentsIter<'a> {
    type Item = BookSegment<'a, AnySegment>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|seg| seg.with_book(self.book))
    }
}

impl<'a> IntoIterator for BookPassageSegments<'a> {
    type Item = BookSegment<'a, AnySegment>;

    type IntoIter = BookPassageSegmentsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BookPassageSegmentsIter {
            book: self.book,
            iter: self.segments.into_iter(),
        }
    }
}

impl<'a> BookPassageSegments<'a> {
    pub fn iter(&self) -> impl Iterator<Item = BookSegment<AnySegment>> + '_ {
        let book = &self.book;
        let it = self.segments.iter().map(move |seg| seg.with_book(*book));
        it
    }

    // pub fn parse(book: u8, segment_input: &str) -> Result<Self, String> {
    //     Ok(Self {
    //         book,
    //         segments: PassageSegments::parse(segment_input)?,
    //     })
    // }

    pub fn overlaps_with(&self, other: &BookPassageSegments) -> bool {
        if self.book.id != other.book.id {
            return false;
        }
        self.segments.contains_overlap(&other.segments)
    }
}
