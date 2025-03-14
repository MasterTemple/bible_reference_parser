use itertools::Either;

use crate::{compare::{BookPassageContent, PassageContent, SegmentCompare}, organizer::PassageOrganizer, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}, segment::BookSegment};
use std::{collections::BTreeMap, fmt::Debug, ops::{Deref, DerefMut}};

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

    pub fn modify<Segment: SegmentCompare>(&mut self, key: BookSegment<Segment>) -> &mut Container {
        self.0.entry(key.book).or_default()
            .modify(key.segment)
    }

    fn iter_book<'a, Segment: SegmentCompare, OutputSegment: SegmentCompare, Return: Iterator<Item = PassageContent<'a, OutputSegment, Container>>>
        (&'a self,
            key: &'a BookSegment<Segment>,
            iter: impl FnOnce(&'a PassageOrganizer<Container>, &'a Segment) -> Return
        )
        -> impl Iterator<Item = BookPassageContent<'a, OutputSegment, Container>>
    {
        match self.0.get(&key.book) {
            Some(org) => Either::Left(iter(org, &key.segment).map(move |psg| psg.with_book(key.book))),
            None => Either::Right(std::iter::empty()),
        }
    }

    // pub fn iter_all_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = BookPassageContent<'a, PassageSegment, Container>> {
    // }

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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {

    }
}
