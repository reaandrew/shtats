use ramhorns::{Content};

#[derive(Content)]
pub struct SummaryViewModel {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Content)]
pub struct KeyValue {
    pub(crate) key: String,
    pub(crate) value: i32,
}

#[derive(Content)]
pub struct LinesValue{
    pub(crate) key: String,
    pub(crate) lines_added: i32,
    pub(crate) lines_deleted: i32
}

#[derive(Content)]
pub struct FilesValue{
    pub(crate) key: String,
    pub(crate) files_added: i32,
    pub(crate) files_deleted: i32,
    pub(crate) files_modified: i32,
    pub(crate) files_renamed: i32
}

#[derive(Content)]
pub struct PunchesValue{
    pub(crate) weekday: u32,
    pub(crate) hour: u32,
    pub(crate) commits: u32
}

#[derive(Content, Default)]
pub struct GitStatsViewModel {
    pub summary: Vec<SummaryViewModel>,
    pub total_commits_by_day: Vec<KeyValue>,
    pub total_lines_by_day: Vec<LinesValue>,
    pub total_files_by_day: Vec<FilesValue>,
    pub punch_data: Vec<PunchesValue>
}