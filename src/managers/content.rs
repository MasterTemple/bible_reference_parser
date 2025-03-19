use crate::{
    books::book_data::BookInfo,
    passage::segment::{any_segment::AnySegment, segment::SegmentFns},
};

#[derive(Copy, Clone, Debug)]
pub struct PassageContent<'a, Segment: SegmentFns, Content> {
    pub segment: Segment,
    pub content: &'a Content,
}

impl<'a, Segment: SegmentFns, Content> PassageContent<'a, Segment, Content> {
    pub fn generalize(self) -> PassageContent<'a, AnySegment, Content> {
        PassageContent {
            segment: self.segment.into(),
            content: self.content,
        }
    }

    pub fn with_book(self, book: BookInfo<'a>) -> BookPassageContent<'a, Segment, Content> {
        BookPassageContent {
            book,
            segment: self.segment,
            content: self.content,
        }
    }
}

// - what about `book: BookSegment`?
// - actually i probably should remove BookPassageContent and just have PassageContent which has both
// book and content
// - but then i have to pass BookInfo down?
#[derive(Copy, Clone, Debug)]
pub struct BookPassageContent<'a, Segment: SegmentFns, Content> {
    pub book: BookInfo<'a>,
    pub segment: Segment,
    pub content: &'a Content,
}

impl<'a, Segment: SegmentFns, Content> BookPassageContent<'a, Segment, Content> {
    pub fn generalize(self) -> BookPassageContent<'a, AnySegment, Content> {
        BookPassageContent {
            book: self.book,
            segment: self.segment.into(),
            content: self.content,
        }
    }
}
