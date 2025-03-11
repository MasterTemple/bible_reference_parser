use bible_reference_parser::{compare::SegmentCompare, organizer::PassageOrganizer, parse::ParsableSegment, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter}, segment::PassageSegment, segments::PassageSegments};
use itertools::Itertools;

// put literally anything here
#[derive(Debug)]
enum MyData {
    Note(String),
    Tag(String),
}

type MyContainer = Vec<MyData>;

fn main() -> Result<(), String> {
    let psg = |p: &str| PassageSegment::parse(p).unwrap();
    // the best practice would be to have one [`PassageOrganizer`] per book
    // but books are beyond the scope of this module
    let mut data = PassageOrganizer::<MyContainer>::new();

    // modify returns a mutable reference to whatever your container is
    // if the container is not initialized, it initializes it 
    *data.modify(psg("1:1")) = vec![MyData::Note("Here is a note on 1:1".into())];
    data.modify(psg("1:2")).push(MyData::Note("Here is a note on 1:2".into()));
    data.modify(psg("1")).push(MyData::Note("Some thoughts on chapter 1".into()));
    data.modify(psg("1:2-3")).push(MyData::Tag("#some-tag".into()));

    // get content of a specific proximity
    println!("{:#?}", data.get_chapter_verse_content(&psg("1:2")));

    // get all content
    println!("{:#?}", data.get_all_content(&psg("1:1-3")).collect_vec());

    // get all content grouped by proximity
    println!("{:#?}", data.get_all_content_grouped(&psg("1:1-3")));


    Ok(())
}
