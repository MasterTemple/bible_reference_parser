use itertools::Either;

use crate::{bible::BibleBookOrganizer, compare::{BookPassageContent, PassageContent, SegmentCompare}, passage_segments::chapter_verse::ChapterVerse, book_segment::BookSegment};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug, Default)]
pub struct BibleVerseOrganizer<Content: Debug + Default> {
    chapter_verse: BTreeMap<u8, BibleBookOrganizer<Content>>,
}

impl<Content: Debug + Default> BibleVerseOrganizer<Content> {
    pub fn new() -> Self {
        Self {
            chapter_verse: BTreeMap::default(),
        }
    }

    pub fn modify(&mut self, key: BookSegment<ChapterVerse>) -> &mut Content {
        self.chapter_verse
            .entry(key.book).or_default()
            .modify(key.segment)
    }

    fn iter_book<'a, Segment: SegmentCompare, OutputSegment: SegmentCompare, Return: Iterator<Item = PassageContent<'a, OutputSegment, Content>>>
        (&'a self,
            key: &'a BookSegment<Segment>,
            iter: impl FnOnce(&'a BibleBookOrganizer<Content>, &'a Segment) -> Return
        )
        -> impl Iterator<Item = BookPassageContent<'a, OutputSegment, Content>>
    {
        match self.chapter_verse.get(&key.book) {
            Some(org) => Either::Left(iter(org, &key.segment).map(move |psg| psg.with_book(key.book))),
            None => Either::Right(std::iter::empty()),
        }
    }

    pub fn iter_segment_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> impl Iterator<Item = BookPassageContent<'a, ChapterVerse, Content>> {
        self.iter_book(key, |org, seg| org.iter_segment_content(seg))
    }
    pub fn get_segment_content<'a, Segment: SegmentCompare>(&'a self, key: &'a BookSegment<Segment>) -> Vec<BookPassageContent<'a, ChapterVerse, Content>> {
        self.iter_segment_content(key).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{compare::SegmentCompare, passage_segments::{chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange}, book_segment::BookSegment};

    use super::BibleVerseOrganizer;

    #[test]
    fn test() {
        let mut bible = BibleVerseOrganizer::<String>::new();
        // *bible.modify(ChapterVerse::new(1, 1).with_book(1)) = String::from("In the beginning God created the heavens and the earth.");
        *bible.modify(BookSegment::chapter_verse(1, 1, 1)) = String::from("In the beginning God created the heavens and the earth.");

        *bible.modify(ChapterVerse::new(1, 1).with_book(43)) = String::from("In the beginning was the Word, and the Word was with God, and the Word was God.");
        *bible.modify(ChapterVerse::new(1, 2).with_book(43)) = String::from("He was in the beginning with God.");
        *bible.modify(ChapterVerse::new(1, 3).with_book(43)) = String::from("All things were made through him, and without him was not any thing made that was made.");

        println!("{:#?}", bible.get_segment_content(&ChapterVerse::new(1, 1).with_book(1)));
        /*
        [
            BookPassageContent {
                book: 1,
                segment: ChapterVerse { chapter: 1, verse: 1, },
                content: "In the beginning God created the heavens and the earth.",
            },
        ]
        */

        println!("{:#?}", bible.get_segment_content(&ChapterVerseRange::new(1, 1, 3).with_book(43)));
        /*
        [
            BookPassageContent {
                book: 43,
                segment: ChapterVerse { chapter: 1, verse: 1, },
                content: "In the beginning was the Word, and the Word was with God, and the Word was God.",
            },
            BookPassageContent {
                book: 43,
                segment: ChapterVerse { chapter: 1, verse: 2, },
                content: "He was in the beginning with God.",
            },
            BookPassageContent {
                book: 43,
                segment: ChapterVerse { chapter: 1, verse: 3, },
                content: "All things were made through him, and without him was not any thing made that was made.",
            },
        ]
        */
    }
}
