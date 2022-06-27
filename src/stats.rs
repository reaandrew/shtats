use serde_json::{Error};
use crate::models::GitCommit;
use crate::viewmodel::GitStatsJsonViewModelItem;

pub trait JsonValue{
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error>;
}

pub trait GitStat : JsonValue{
    fn process(&mut self, commit: &GitCommit);
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
pub struct LineStatsAverage {
    pub(crate) avg_added: f64,
    pub(crate) avg_deleted: f64,
}

#[derive(Default, Clone, PartialEq)]
pub struct FileStats {
    pub(crate) added: i32,
    pub(crate) modified: i32,
    pub(crate) deleted: i32,
    pub(crate) renamed: i32,
    pub(crate) copied: i32,
    pub(crate) type_changed: i32,
    pub(crate) unmerged: i32,
    pub(crate) unknown: i32,
    pub(crate) pairing_broken: i32
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
pub struct PunchStats{
    pub(crate) weekday: u32,
    pub(crate) hour: u32,
    pub(crate) commits: u32,
}
