use bible_reference_parser::{compare::SegmentCompare, organizer::PassageOrganizer, parse::ParsableSegment, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter}, segment::PassageSegment, segments::PassageSegments};

fn main() -> Result<(), String> {
    // Create specific passage reference segment type
    ChapterVerseRange::new(1, 2, 3);
    ChapterVerseRange::parse("1:2-3")?;
    let _: ChapterVerseRange = "1:2-3".parse()?;

    // Create as generic passage segment
    PassageSegment::ChapterVerseRange(ChapterVerseRange::new(1, 2, 3));
    PassageSegment::chapter_verse_range(1, 2, 3);
    let _: PassageSegment = ChapterVerseRange::new(1, 2, 3).into();
    PassageSegment::parse("1:2-3")?;
    let _: PassageSegment = "1:2-3".parse()?;

    // Coerce segment types into other compatible types
    // coerce to a wider scope: John 1:2 -> John 1
    FullChapter::try_from(PassageSegment::chapter_verse(1, 2))?; // FullChapter
    // coerce to actual type of a segment: John 1:2-2 -> John 1:2
    ChapterVerseRange::parse("1:2-2")?.actual(); // ChapterVerse

    // Parse multiple contextually-joined segments
    PassageSegments::parse("1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8")?;
    PassageSegments::parse("   1, 2-4; 5:1–3,5,7—9,12—6:6,  7:7⸺  8:8, ")?;

    Ok(())
}
