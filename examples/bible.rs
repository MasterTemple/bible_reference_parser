use bible_reference_parser::{bible::BibleBookOrganizer, parse::ParsableSegment, passage_segments::chapter_verse::ChapterVerse, segment::PassageSegment};

fn main() -> Result<(), String> {
    let psg = |p: &str| PassageSegment::parse(p).unwrap();
    let verse = |v: &str| ChapterVerse::parse(v).unwrap();

    // just using strings for the most basic type of Bible content
    let mut john = BibleBookOrganizer::<String>::new();

    // Only chapter:verse pairs can be modified
    *john.modify(verse("1:1")) =
        String::from("In the beginning was the Word, and the Word was with God, and the Word was God.");
    *john.modify(verse("1:2")) =
        String::from("He was in the beginning with God.");
    *john.modify(verse("1:3")) =
        String::from("All things were made through him, and without him was not any thing made that was made.");
    
    // but any kind of passage can access the content
    println!("{:#?}", john.get_chapter_verse_content(&psg("1:1")));
    /* [
        (
            ChapterVerse { chapter: 1, verse: 1, },
            "In the beginning was the Word, and the Word was with God, and the Word was God.",
        ),
    ] */

    println!("{:#?}", john.get_chapter_verse_content(&psg("1:1-3")));
    /* [
        (
            ChapterVerse { chapter: 1, verse: 1, },
            "In the beginning was the Word, and the Word was with God, and the Word was God.",
        ),
        (
            ChapterVerse { chapter: 1, verse: 2, },
            "He was in the beginning with God.",
        ),
        (
            ChapterVerse { chapter: 1, verse: 3, },
            "All things were made through him, and without him was not any thing made that was made.",
        ),
    ] */

    Ok(())

}
