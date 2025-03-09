use std::{fmt::Debug, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{parse::{match_and_sanitize_segment_input, parse_reference_segments}, segment::PassageSegment};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassageSegments(pub Vec<PassageSegment>);

impl Deref for PassageSegments {
    type Target = Vec<PassageSegment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PassageSegments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PassageSegments {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn try_parse(segment_input: &str) -> Option<Self> {
        let input = match_and_sanitize_segment_input(segment_input)?;
        let segments = parse_reference_segments(&input);
        Some(segments)
    }
}
