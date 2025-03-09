use std::{fmt::Debug, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::segment::PassageSegment;

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
}
