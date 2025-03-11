pub trait SegmentCompare {
    fn get_starting_verse(&self) -> u8;

    fn get_starting_chapter(&self) -> u8;

    fn get_ending_verse(&self) -> Option<u8>;

    fn get_ending_chapter(&self) -> u8;
}

// impl<T: SegmentCompare> PartialOrd for T {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         todo!()
//     }
// }
