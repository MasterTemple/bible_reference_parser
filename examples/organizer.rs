use std::collections::BTreeMap;

use bible_reference_parser::{compare::SegmentCompare, organizer::PassageOrganizer, parse::ParsableSegment, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter}, segment::PassageSegment, segments::PassageSegments};
use itertools::Itertools;
use serde::Serialize;

// put literally anything here
#[derive(Debug, Serialize)]
enum MyData {
    Note(String),
    Tag(String),
}

type MyContainer = Vec<MyData>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let psg = |p: &str| PassageSegment::parse(p).unwrap();
    // the best practice would be to have one [`PassageOrganizer`] per book
    // but books are beyond the scope of this module
    let mut data = PassageOrganizer::<MyContainer>::new();

    // modify returns a mutable reference to whatever your container is
    // if the container is not initialized, it initializes it 
    *data.modify(psg("1:1")) = vec![MyData::Note("Here is a note on 1:1".into())];
    data.modify(psg("1:2")).push(MyData::Note("Here is a note on 1:2".into()));
    data.modify(psg("1:2")).push(MyData::Note("Here is another note on 1:2".into()));
    data.modify(psg("1")).push(MyData::Note("Some thoughts on chapter 1".into()));
    data.modify(psg("1:2-3")).push(MyData::Tag("#some-tag".into()));

    // get content of a specific proximity
    println!("{:#?}", data.get_chapter_verse_content(&psg("1:2")));
    /* [
      (
        ChapterVerse { chapter: 1, verse: 2 },
        [
            Note("Here is a note on 1:2"),
            Note("Here is another note on 1:2"),
        ],
      )
    ] */

    // get all content
    println!("{:#?}", data.get_all_content(&psg("1:1-3")));
    /* [
        (
            ChapterVerse(ChapterVerse { chapter: 1, verse: 1 }),
            [ Note("Here is a note on 1:1"), ],
        ),
        (
            ChapterVerse(ChapterVerse { chapter: 1, verse: 2 }),
            [
                Note("Here is a note on 1:2"),
                Note("Here is another note on 1:2"),
            ],
        ),
        (
            ChapterVerseRange(ChapterVerseRange { chapter: 1, verses: RangePair { start: 2, end: 3 } }),
            [ Tag("#some-tag"), ],
        ),
        (
            FullChapter(FullChapter { chapter: 1 }),
            [ Note("Some thoughts on chapter 1"), ],
        ),
    ] */

    // get all content grouped by proximity
    println!("{:#?}", data.get_all_content_grouped(&psg("1:1-3")));
    /* GroupedContent {
        chapter_verse: [
            (
                ChapterVerse { chapter: 1, verse: 1 },
                [ Note("Here is a note on 1:1"), ],
            ),
            (
                ChapterVerse { chapter: 1, verse: 2 },
                [
                    Note("Here is a note on 1:2"),
                    Note("Here is another note on 1:2"),
                ],
            ),
        ],
        chapter_verse_range: [
            (
                ChapterVerseRange { chapter: 1, verses: RangePair { start: 2, end: 3 } },
                [
                    Tag("#some-tag"),
                ],
            ),
        ],
        chapter_range: [],
        full_chapter: [
            (
                FullChapter { chapter: 1 },
                [ Note("Some thoughts on chapter 1"), ],
            ),
        ],
        full_chapter_range: [],
    }
    */

    // one way to get all content of a passage segment list
    let passage = PassageSegments::parse("1:1,3-4")?;
    let map: BTreeMap<_, _> = passage.iter().map(|psg| (psg, data.get_all_content(psg))).collect();
    println!("{map:#?}");
    /* {
        ChapterVerse( ChapterVerse { chapter: 1, verse: 1 } ): [
            (
                ChapterVerse( ChapterVerse { chapter: 1, verse: 1 } ),
                [
                    Note( "Here is a note on 1:1" ),
                ],
            ),
            (
                FullChapter( FullChapter { chapter: 1 } ),
                [
                    Note( "Some thoughts on chapter 1" ),
                ],
            ),
        ],
        ChapterVerseRange( ChapterVerseRange { chapter: 1, verses: RangePair { start: 3, end: 4 } } ): [
            (
                ChapterVerseRange( ChapterVerseRange { chapter: 1, verses: RangePair { start: 2, end: 3 } } ),
                [
                    Tag( "#some-tag" ),
                ],
            ),
            (
                FullChapter( FullChapter { chapter: 1 } ),
                [
                    Note( "Some thoughts on chapter 1" ),
                ],
            ),
        ],
    } */

    Ok(())
}
