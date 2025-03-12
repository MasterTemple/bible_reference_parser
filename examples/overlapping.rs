use bible_reference_parser::{compare::SegmentCompare, organizer::PassageOrganizer, parse::ParsableSegment, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter}, segment::PassageSegment, segments::PassageSegments};

fn main() -> Result<(), String> {
    let psg = |p: &str| PassageSegment::parse(p).unwrap(); // shorthand for single passage
    let display_overlap = |p1: PassageSegment, p2: PassageSegment| println!("{p1} overlaps with {p2}: '{}'", p1.overlaps_with(&p2));

    // check individual segment overlap
    display_overlap(psg("1:1"), psg("1:2")); // false
    display_overlap(psg("1:1-3"), psg("1:2")); // true
    display_overlap(psg("1:1-3"), psg("1:3-4")); // true

    let first = PassageSegments::parse("1:1-3, 4; 2:1-3")?;
    let second = PassageSegments::parse("2:2-8; 3:1-4:1")?;
    let third = PassageSegments::parse("2:9-12")?;

    // check if segment list has any overlap with a single segment
    println!("{}", first.overlaps_with(&psg("1:1"))); // true
    println!("{}", second.overlaps_with(&psg("1:1"))); // false

    // check for any overlapping segments between segment list
    println!("{}", first.contains_overlap(&second)); // true
    println!("{}", first.contains_overlap(&third)); // false
    println!("{}", second.contains_overlap(&third)); // false

    Ok(())
}
