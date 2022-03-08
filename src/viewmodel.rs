use std::collections::HashMap;
use ramhorns::{Content};
use serde_json::Value;
use serde::Serialize;

#[derive(Serialize)]
pub struct SummaryViewModelItem {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Serialize)]
pub struct KeyValue {
    pub(crate) key: String,
    pub(crate) value: i32,
}

#[derive(Serialize)]
pub struct LinesValue{
    pub(crate) key: String,
    pub(crate) lines_added: i32,
    pub(crate) lines_deleted: i32
}

#[derive(Serialize)]
pub struct FilesValue{
    pub(crate) key: String,
    pub(crate) files_added: i32,
    pub(crate) files_deleted: i32,
    pub(crate) files_modified: i32,
    pub(crate) files_renamed: i32
}

#[derive(Serialize)]
pub struct PunchesValue{
    pub(crate) weekday: u32,
    pub(crate) hour: u32,
    pub(crate) commits: u32
}

#[derive(Default, Serialize)]
pub struct GitStatsJsonViewModel {
    pub(crate) summary: Vec<Value>,
    pub(crate) data: HashMap<String, Value>
}

#[derive(Default)]
pub struct GitStatsJsonViewModelItem {
    pub(crate) summary: Vec<SummaryViewModelItem>,
    pub(crate) key: String,
    pub(crate) data: Value
}

// {
//     pub summary: Vec<SummaryViewModel>,
//     pub total_commits_by_day: Vec<KeyValue>,
//     pub total_lines_by_day: Vec<LinesValue>,
//     pub total_files_by_day: Vec<FilesValue>,
//     pub punch_data: Vec<PunchesValue>
// }