use std::fmt::Debug;
use std::collections::BTreeMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::passage::segment::{segment::SegmentCompare, types::chapter_verse::ChapterVerse};

use super::content::PassageContent;


// pub struct PassageContent<'a, Segment: SegmentCompare, Content> {
//     segment: Segment,
//     content: &'a Content
// }

/// - This is a struct like [`PassageOrganizer`], but is meant to store the content of the Bible
/// - It only stores content in chapter:verse segments, because that is how 
/// - Any format is allowed: simple use cases might just use a string for the whole verse, more
/// complex ones might use enums to indicate headers, indentation, and so on, or structs to store
/// an interlinear Bible
/// - This is not meant to store related media
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BibleBookOrganizer<Content: Debug + Default> {
    /// `map[chapter][verse] -> Content`
    chapter_verse: BTreeMap<u8, BTreeMap<u8, Content>>,
}

impl<Content: Debug + Default> BibleBookOrganizer<Content> {
    pub fn new() -> Self {
        Self {
            chapter_verse: BTreeMap::default(),
        }
    }

    pub fn modify(&mut self, seg: ChapterVerse) -> &mut Content {
        self.chapter_verse.entry(seg.chapter).or_default()
            .entry(seg.verse).or_default()
    }

    pub fn get_segment_content<'a>(&'a self, key: &'a impl SegmentCompare) -> Vec<PassageContent<'a, ChapterVerse, Content>>  {
        self.iter_segment_content(key).collect_vec()
    }

    pub fn iter_segment_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, ChapterVerse, Content>> {
        self.chapter_verse.range(key.chapter_range()).flat_map(|(&chapter, map)| {
            map.range(key.verse_range(chapter))
                .map(move|(&verse, content)| ChapterVerse::new(chapter, verse).with_content(content))
        })
    }
}


// #[cfg(test)]
// mod tests {
//     use crate::{parse::ParsableSegment, passage_segments::chapter_verse_range::ChapterVerseRange};
//
//     use super::*;
//
//     #[test]
//     fn chapter_verse() -> Result<(), String> {
//         let mut john = BibleBookOrganizer::<String>::new();
//         *john.modify(ChapterVerse::parse("1:1")?) =
//             String::from("In the beginning was the Word, and the Word was with God, and the Word was God.");
//         *john.modify(ChapterVerse::parse("1:2")?) =
//             String::from("He was in the beginning with God.");
//         *john.modify(ChapterVerse::parse("1:3")?) =
//             String::from("All things were made through him, and without him was not any thing made that was made.");
//         
//         println!("{:#?}", john.get_segment_content(&ChapterVerse::parse("1:1")?));
//         /* [
//             (
//                 ChapterVerse { chapter: 1, verse: 1, },
//                 "In the beginning was the Word, and the Word was with God, and the Word was God.",
//             ),
//         ] */
//
//         println!("{:#?}", john.get_segment_content(&ChapterVerseRange::parse("1:1-3")?));
//         /* [
//             (
//                 ChapterVerse { chapter: 1, verse: 1, },
//                 "In the beginning was the Word, and the Word was with God, and the Word was God.",
//             ),
//             (
//                 ChapterVerse { chapter: 1, verse: 2, },
//                 "He was in the beginning with God.",
//             ),
//             (
//                 ChapterVerse { chapter: 1, verse: 3, },
//                 "All things were made through him, and without him was not any thing made that was made.",
//             ),
//         ] */
//
//         Ok(())
//     }
// }
