use bible_reference_parser::{compare::SegmentCompare, parse::ParsableSegment, passage_segments::{chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter}, segment::PassageSegment, segments::PassageSegments};

#[allow(unused_variables)]
fn main() -> Result<(), String> {
    // Create specific passage reference segment type
    let verse = ChapterVerseRange::new(1, 2, 3);
    let verse = ChapterVerseRange::parse("1:2-3")?;
    let verse: ChapterVerseRange = "1:2-3".parse()?;

    println!("{verse:#?}"); // ChapterVerseRange { chapter: 1, verses: RangePair { start: 2, end: 3, }, }

    // Create as generic passage segment
    let seg = PassageSegment::ChapterVerseRange(ChapterVerseRange::new(1, 2, 3));
    let seg = PassageSegment::chapter_verse_range(1, 2, 3);
    let seg: PassageSegment = ChapterVerseRange::new(1, 2, 3).into();
    let seg = PassageSegment::parse("1:2-3")?;
    let seg: PassageSegment = "1:2-3".parse()?;

    println!("{seg:#?}"); // ChapterVerseRange( ChapterVerseRange { chapter: 1, verses: RangePair { start: 2, end: 3, } } )

    // Coerce segment types into other compatible types
    // coerce to a wider scope: John 1:2 -> John 1
    let chapter = FullChapter::try_from(PassageSegment::chapter_verse(1, 2))?; // FullChapter
    println!("{chapter:#?}"); // FullChapter { chapter: 1 }

    // coerce to actual type of a segment: John 1:2-2 -> John 1:2
    let verse = ChapterVerseRange::parse("1:2-2")?.actual(); // ChapterVerse
    println!("{verse:#?}"); // ChapterVerse( ChapterVerse { chapter: 1, verse: 2 } )

    // Parse multiple contextually-joined segments
    let segments = PassageSegments::parse("1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8")?;
    let segments = PassageSegments::parse("   1, 2-4; 5:1–3,5,7—9,12—6:6,  7:7⸺  8:8, ")?;

    println!("{segments:#?}");
    /* PassageSegments([
        FullChapter( FullChapter { chapter: 1, },),
        FullChapterRange( FullChapterRange( RangePair { start: FullChapter { chapter: 2, }, end: FullChapter { chapter: 4, } } ) ),
        ChapterVerseRange( ChapterVerseRange { chapter: 5, verses: RangePair { start: 1, end: 3, } } ),
        ChapterVerse( ChapterVerse { chapter: 5, verse: 5 } ),
        ChapterVerseRange( ChapterVerseRange { chapter: 5, verses: RangePair { start: 7, end: 9, } } ),
        ChapterRange( ChapterRange( RangePair { start: ChapterVerse { chapter: 5, verse: 12, }, end: ChapterVerse { chapter: 6, verse: 6, } } ) ),
        ChapterRange( ChapterRange( RangePair { start: ChapterVerse { chapter: 7, verse: 7, }, end: ChapterVerse { chapter: 8, verse: 8, } } ) ),
    ]) */

    Ok(())
}
