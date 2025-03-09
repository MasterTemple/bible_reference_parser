// use crate::{
//     // api::BibleAPI,
//     // formatter::{
//     //     ChapterFormatParameters, PassageFormatParameters, PassageFormatter,
//     //     SegmentFormatParameters, Template, VerseFormatParameters,
//     // },
//     segments::{PassageChapterRange, PassageSegment, PassageSegments},
// };
//
// pub trait PassageDataProvider {
//     fn get_book_name(&self, book: usize) -> String;
//     fn get_content(&self, book: usize, chapter: usize, verse: usize) -> String;
//     fn get_ending_chapter(&self, book: usize) -> usize;
//     fn get_ending_verse(&self, book: usize, chapter: usize) -> usize;
// }
//
// #[derive(Clone, Debug)]
// pub struct Passage {
//     // pub range: Range,
//     pub book_id: usize,
//     pub segments: PassageSegments,
// }
//
// impl Passage {
//     /// This should only be called after finding a match in a range
//     // pub fn new(book_id: usize, range: Range, segment_input: &str) -> Self {
//     //     // split into book name and segments
//     //     // get book id
//     //     let segments = PassageSegments::parse(segment_input);
//     //     Self {
//     //         range,
//     //         book_id,
//     //         segments,
//     //     }
//     // }
//
//     pub fn new(book_id: usize, segment_input: &str) -> Self {
//         // split into book name and segments
//         // get book id
//         let segments = PassageSegments::parse(segment_input);
//         Self { book_id, segments }
//     }
//
//     // pub fn new_book(book_id: usize, api: &BibleAPI) -> Self {
//     //     let end_chapter = api.data().get_book_chapter_count(book_id).unwrap_or(1);
//     //     let end_verse = api
//     //         .data()
//     //         .get_chapter_verse_count(book_id, end_chapter)
//     //         .unwrap_or(1);
//     //     let segments = PassageSegments(vec![PassageSegment::PassageChapterRange(
//     //         PassageChapterRange {
//     //             start_chapter: 1,
//     //             end_chapter,
//     //             start_verse: 1,
//     //             end_verse,
//     //         },
//     //     )]);
//     //     Self { book_id, segments }
//     // }
//
//     /// Formats into something like `Ephesians 1:1-4, 5-7, 2:2-3:4, 6`
//     // pub fn full_ref_label(&self, api: &BibleAPI) -> String {
//     //     let book_name = api
//     //         .get_book_name(self.book_id)
//     //         .expect("A Passage struct should not be created if the book_id is invalid.");
//     //     format!("{} {}", book_name, self.segments.label())
//     // }
//
//     /**
//     Returns text like the following:
//
//     ```text
//     [1:1] Paul, an apostle of Christ Jesus by the will of God, To the saints who are in Ephesus, and are faithful in Christ Jesus:
//     [1:2] Grace to you and peace from God our Father and the Lord Jesus Christ.
//     [1:3] Blessed be the God and Father of our Lord Jesus Christ, who has blessed us in Christ with every spiritual blessing in the heavenly places,
//     [1:4] even as he chose us in him before the foundation of the world, that we should be holy and blameless before him. In love
//     ```
//     */
//     // pub fn format(&self, api: &BibleAPI, formatter: &PassageFormatter) -> String {
//     //     // let formatter = api.formatter();
//     //     // let book = "Ephesians";
//     //     // im pretty sure its guaranteed that if it parsed it is a valid book id
//     //     let book = &api.data().get_book_name(self.book_id).unwrap();
//     //     // let segment_template = Template::from_template("{verses}").unwrap();
//     //     // let chapter_template = Template::from_template("\n[{chapter}] {verses}").unwrap();
//     //     // let verse_template = Template::from_template("[{chapter}:{verse}] {content}").unwrap();
//     //
//     //     // i could cache these better
//     //     let segment_template = Template::from_template(&formatter.segment).unwrap();
//     //     let chapter_template = Template::from_template(&formatter.chapter).unwrap();
//     //     let verse_template = Template::from_template(&formatter.verse).unwrap();
//     //     let passage_template = Template::from_template(&formatter.text).unwrap();
//     //
//     //     let segment_range_content = self
//     //         .segments
//     //         .iter()
//     //         .map(|seg| {
//     //             let chapter_range_content = (seg.get_starting_chapter()..=seg.get_ending_chapter())
//     //                 .map(|chapter| {
//     //                     let start_verse = if chapter == seg.get_starting_chapter() {
//     //                         seg.get_starting_verse()
//     //                     } else {
//     //                         1
//     //                     };
//     //                     let end_verse = if chapter == seg.get_ending_chapter() {
//     //                         seg.get_ending_verse()
//     //                     } else {
//     //                         api.data()
//     //                             .get_chapter_verse_count(self.book_id, chapter)
//     //                             .unwrap_or(0)
//     //                     };
//     //                     let verse_range_content = (start_verse..=end_verse)
//     //                         .filter_map(|verse| {
//     //                             let content =
//     //                                 api.data()
//     //                                     .get_bible_contents(self.book_id, chapter, verse)?;
//     //
//     //                             let params = VerseFormatParameters {
//     //                                 book,
//     //                                 chapter,
//     //                                 verse,
//     //                                 // content: &format!("content-{chapter}-{verse}"),
//     //                                 content: &content,
//     //                             };
//     //                             Some(verse_template.fill(&params).unwrap())
//     //                         })
//     //                         .collect::<Vec<_>>()
//     //                         .join(&formatter.join_verses);
//     //
//     //                     let params = ChapterFormatParameters {
//     //                         book,
//     //                         chapter,
//     //                         start_verse,
//     //                         end_verse,
//     //                         verses: &verse_range_content,
//     //                     };
//     //                     chapter_template.fill(&params).unwrap()
//     //                 })
//     //                 .collect::<Vec<_>>()
//     //                 // this will not work how you expect
//     //                 // because segments are outside of chapters
//     //                 .join(&formatter.join_chapters);
//     //             // format!("{} {chapter_range_content}", seg.label())
//     //             let params = SegmentFormatParameters {
//     //                 book,
//     //                 label: &seg.label(),
//     //                 verses: &chapter_range_content,
//     //             };
//     //             segment_template.fill(&params).unwrap()
//     //         })
//     //         .collect::<Vec<_>>()
//     //         .join(&formatter.join_segments);
//     //
//     //     let params = PassageFormatParameters {
//     //         book,
//     //         segments: &segment_range_content,
//     //         label: &self.segments.label(),
//     //     };
//     //     passage_template.fill(&params).unwrap()
//     // }
//
//     // pub fn format_closure(
//     //     &self,
//     //     formatter: PassageFormatter,
//     //     book: &str,
//     //     get_content: impl Fn(usize, usize) -> String,
//     // ) -> String {
//     //     // let book = "Ephesians";
//     //     // let segment_template = Template::from_template("{verses}").unwrap();
//     //     // let chapter_template = Template::from_template("\n[{chapter}] {verses}").unwrap();
//     //     // let verse_template = Template::from_template("[{chapter}:{verse}] {content}").unwrap();
//     //
//     //     let segment_template = Template::from_template(&formatter.segment).unwrap();
//     //     let chapter_template = Template::from_template(&formatter.chapter).unwrap();
//     //     let verse_template = Template::from_template(&formatter.verse).unwrap();
//     //     let passage_template = Template::from_template(&formatter.text).unwrap();
//     //
//     //     let segment_range_content = self
//     //         .segments
//     //         .iter()
//     //         .map(|seg| {
//     //             let chapter_range_content = (seg.get_starting_chapter()..=seg.get_ending_chapter())
//     //                 .map(|chapter| {
//     //                     let start_verse = if chapter == seg.get_starting_chapter() {
//     //                         seg.get_starting_verse()
//     //                     } else {
//     //                         1
//     //                     };
//     //                     let end_verse = if chapter == seg.get_ending_chapter() {
//     //                         seg.get_ending_verse()
//     //                     } else {
//     //                         10 // calc
//     //                     };
//     //                     let verse_range_content = (start_verse..=end_verse)
//     //                         .map(|verse| {
//     //                             let params = VerseFormatParameters {
//     //                                 book,
//     //                                 chapter,
//     //                                 verse,
//     //                                 content: &get_content(chapter, verse),
//     //                             };
//     //                             verse_template.fill(&params).unwrap()
//     //                         })
//     //                         .collect::<Vec<_>>()
//     //                         .join(&formatter.join_verses);
//     //
//     //                     let params = ChapterFormatParameters {
//     //                         book,
//     //                         chapter,
//     //                         start_verse,
//     //                         end_verse,
//     //                         verses: &verse_range_content,
//     //                     };
//     //                     chapter_template.fill(&params).unwrap()
//     //                 })
//     //                 .collect::<Vec<_>>()
//     //                 // this will not work how you expect
//     //                 // because segments are outside of chapters
//     //                 .join(&formatter.join_chapters);
//     //             // format!("{} {chapter_range_content}", seg.label())
//     //             let params = SegmentFormatParameters {
//     //                 book,
//     //                 label: &seg.label(),
//     //                 verses: &chapter_range_content,
//     //             };
//     //             segment_template.fill(&params).unwrap()
//     //         })
//     //         .collect::<Vec<_>>()
//     //         .join(&formatter.join_segments);
//     //
//     //     let params = PassageFormatParameters {
//     //         book,
//     //         segments: &segment_range_content,
//     //         label: &self.segments.label(),
//     //     };
//     //     passage_template.fill(&params).unwrap()
//     // }
// }
