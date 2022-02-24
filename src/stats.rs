use std::collections::HashMap;
use crate::{GitCommit};
use crate::duplicates::DuplicateDetector;

pub trait GitStat {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats);
}


#[derive(Default, Clone, PartialEq)]
pub struct SummaryStats {
    pub(crate) commit_count: i32,
    pub(crate) date_first_commit: String,
    pub(crate) first_committer: String,
    pub(crate) total_lines_added: i32,
    pub(crate) total_lines_deleted: i32,
    total_files_added: i32,
    total_files_deleted: i32,
    total_files_modified: i32,
    // TODO: Add renames
}

#[derive(Default, Clone, PartialEq)]
pub struct LineStats {
    pub(crate) added: i32,
    pub(crate) deleted: i32,
}

#[derive(Default, Clone, PartialEq)]
pub struct FileStats {
    pub(crate) added: i32,
    pub(crate) modified: i32,
    pub(crate) deleted: i32,
    pub(crate) renamed: i32,
}

#[derive(Default, Clone, PartialEq)]
pub struct MessageStats {
    pub(crate) max_size: i32,
    pub(crate) max_lines: i32,
    pub(crate) avg_size: i32,
    pub(crate) avg_lines: i32,
    pub(crate) min_size: i32,
    pub(crate) min_lines: i32,
}

#[derive(Default, Clone, PartialEq)]
pub struct GitStats {
    pub(crate) count: i32,
    pub(crate) summary: SummaryStats,
    pub(crate) total_commits_by_day: HashMap<String, i32>,
    pub(crate) total_lines_by_day: HashMap<String, LineStats>,
    pub(crate) total_files_by_day: HashMap<String, FileStats>,
    pub(crate) total_message_lines: i32,
    pub(crate) total_message_size: i32,
    pub(crate) message_stats: MessageStats,
    pub(crate) dup_detector: DuplicateDetector
}

impl GitStats{
    pub(crate) fn new(threshold: i32) -> Self {
        return GitStats{
            count: 0,
            summary: Default::default(),
            total_commits_by_day: Default::default(),
            total_lines_by_day: Default::default(),
            total_files_by_day: Default::default(),
            total_message_lines: 0,
            total_message_size: 0,
            message_stats: Default::default(),
            dup_detector: DuplicateDetector::new(threshold)
        }
    }
}
