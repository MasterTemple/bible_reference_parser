use itertools::Either;

use std::{collections::BTreeMap, fmt::Debug, ops::{Deref, DerefMut}};

use crate::{books::book_segment::BookSegment, passage::segment::{any_segment::PassageSegment, segment::SegmentCompare, types::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}}};

use super::{content::{BookPassageContent, PassageContent}, segment_organizer::PassageOrganizer};

/// This is meant to organize content across the entire Bible
/// For something to store only verse content instead of all related data, see [`BibleVerseOrganizer`](bible_reference_parser::bible_verse_organizer::BibleVerseOrganizer)
#[derive(Debug, Default)]
pub struct FullBibleOrganizer<Container: Debug + Default>(
    BTreeMap<u8, PassageOrganizer<Container>>,
);

impl<Container: Debug + Default> Deref for FullBibleOrganizer<Container> {
    type Target = BTreeMap<u8, PassageOrganizer<Container>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Container: Debug + Default> DerefMut for FullBibleOrganizer<Container> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Container: Debug + Default> FullBibleOrganizer<Container> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    // pub fn modify<Segment: SegmentCompare>(&mut self, key: BookSegment<Segment>) -> &mut Container {
    //     self.0.entry(key.book).or_default()
    //         .modify(key.segment)
    // }

    pub fn modify(&mut self, key: BookSegment<ChapterVerse>) -> &mut Container {
        self.0.entry(key.book.id).or_default()
            .modify(key.segment)
    }

    fn iter_book<'a, Segment: SegmentCompare, OutputSegment: SegmentCompare, Return: Iterator<Item = PassageContent<'a, OutputSegment, Container>>>
        (&'a self,
            key: &'a BookSegment<Segment>,
            iter: impl FnOnce(&'a PassageOrganizer<Container>, &'a Segment) -> Return
        )
        -> impl Iterator<Item = BookPassageContent<'a, OutputSegment, Container>>
    {
        match self.0.get(&key.book.id) {
            Some(org) => Either::Left(iter(org, &key.segment).map(move |psg| psg.with_book(key.book))),
            None => Either::Right(std::iter::empty()),
        }
    }

    pub fn iter_all_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, PassageSegment, Container>> {
        self.iter_chapter_verse_content(key).map(|psg| psg.generalize())
            .chain(self.iter_chapter_verse_range_content(key).map(|psg| psg.generalize()))
            .chain(self.iter_chapter_range_content(key).map(|psg| psg.generalize()))
            .chain(self.iter_full_chapter_content(key).map(|psg| psg.generalize()))
            .chain(self.iter_full_chapter_range_content(key).map(|psg| psg.generalize()))
    }
    pub fn get_all_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, PassageSegment, Container>> {
        self.iter_all_content(key).collect()
    }

    pub fn get_all_content_grouped<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> BookGroupedContent<'a, Container> {
        BookGroupedContent {
            chapter_verse: self.get_chapter_verse_content(key),
            chapter_verse_range: self.get_chapter_verse_range_content(key),
            chapter_range: self.get_chapter_range_content(key),
            full_chapter: self.get_full_chapter_content(key),
            full_chapter_range: self.get_full_chapter_range_content(key),
        }
    }

}

#[derive(Clone, Debug, Default)]
pub struct BookGroupedContent<'a, Container: Debug + Default> {
    pub chapter_verse: Vec<BookPassageContent<'a, ChapterVerse, Container>>,
    pub chapter_verse_range: Vec<BookPassageContent<'a, ChapterVerseRange, Container>>,
    pub chapter_range: Vec<BookPassageContent<'a, ChapterRange, Container>>,
    pub full_chapter: Vec<BookPassageContent<'a, FullChapter, Container>>,
    pub full_chapter_range: Vec<BookPassageContent<'a, FullChapterRange, Container>>,
}

impl<Container: Debug + Default> FullBibleOrganizer<Container> {
    pub fn iter_chapter_verse_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, ChapterVerse, Container>> {
        self.iter_book(key, |org, seg| org.iter_chapter_verse_content(seg))
    }
    pub fn get_chapter_verse_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, ChapterVerse, Container>> {
        self.iter_chapter_verse_content(key).collect()
    }

    pub fn iter_chapter_verse_range_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, ChapterVerseRange, Container>> {
        self.iter_book(key, |org, seg| org.iter_chapter_verse_range_content(seg))
    }
    pub fn get_chapter_verse_range_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, ChapterVerseRange, Container>> {
        self.iter_chapter_verse_range_content(key).collect()
    }

    pub fn iter_chapter_range_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, ChapterRange, Container>> {
        self.iter_book(key, |org, seg| org.iter_chapter_range_content(seg))
    }
    pub fn get_chapter_range_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, ChapterRange, Container>> {
        self.iter_chapter_range_content(key).collect()
    }

    pub fn iter_full_chapter_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, FullChapter, Container>> {
        self.iter_book(key, |org, seg| org.iter_full_chapter_content(seg))
    }
    pub fn get_full_chapter_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, FullChapter, Container>> {
        self.iter_full_chapter_content(key).collect()
    }

    pub fn iter_full_chapter_range_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, FullChapterRange, Container>> {
        self.iter_book(key, |org, seg| org.iter_full_chapter_range_content(seg))
    }
    pub fn get_full_chapter_range_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, FullChapterRange, Container>> {
        self.iter_full_chapter_range_content(key).collect()
    }

}

// #[cfg(test)]
// mod tests {
//     use crate::{compare::SegmentCompare, parse::ParsableSegment, passage_segments::chapter_verse::ChapterVerse};
//
//     use super::FullBibleOrganizer;
//
//     #[test]
//     fn test() -> Result<(), String> {
//         // let mut bible = FullBibleOrganizer::<String>::new();
//         // *bible.modify(ChapterVerse::parse("1:1")?.with_book(43)) =
//         //     String::from("In the beginning was the Word, and the Word was with God, and the Word was God.");
//         // *bible.modify(ChapterVerse::parse("1:2")?.with_book(43)) =
//         //     String::from("He was in the beginning with God.");
//         // *bible.modify(ChapterVerse::parse("1:3")?.with_book(43)) =
//         //     String::from("All things were made through him, and without him was not any thing made that was made.");
//         //
//         // println!("{:#?}", bible.get_chapter_verse_content(&ChapterVerse::parse("1:1")?.with_book(43)));
//         /*
//         [
//             BookPassageContent {
//                 book: 43,
//                 segment: ChapterVerse { chapter: 1, verse: 1, },
//                 content: "In the beginning was the Word, and the Word was with God, and the Word was God.",
//             },
//         ]
//         */
//
//         Ok(())
//     }
// }
