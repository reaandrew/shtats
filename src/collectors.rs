use crate::collectors::commits_by_day::CommitsByDayCollector;
use crate::collectors::commits_by_file_extension::CommitsByFileExtension;
use crate::collectors::files_by_commits::FilesByCommitsCollector;
use crate::collectors::files_by_day::FilesByDayCollector;
use crate::collectors::files_by_lines::FilesByLines;
use crate::collectors::lines_by_day::LinesByDayCollector;
use crate::collectors::messages::MessagesCollector;
use crate::collectors::punch_card::PunchCardCollector;
use crate::collectors::summary_stats::SummaryStatsCollector;
use crate::collectors::user_summary::UserSummaryCollector;
use crate::GitStat;

pub mod summary_stats;
pub mod commits_by_day;
pub mod lines_by_day;
pub mod files_by_day;
pub mod messages;
pub mod similar_files;
pub mod punch_card;
pub mod files_by_commits;
pub mod files_by_lines;
pub mod commits_by_file_extension;
pub mod user_summary;

pub fn create_stat_collectors() -> Vec<Box<dyn GitStat>> {
    let stats_functions: Vec<Box<dyn GitStat>> = vec![
        Box::new(SummaryStatsCollector::default()),
        Box::new(CommitsByDayCollector::default()),
        Box::new(LinesByDayCollector::default()),
        Box::new(MessagesCollector::default()),
        //Box::new(SimilarFilesChangingCollector::default()),
        Box::new(FilesByDayCollector::default()),
        Box::new(PunchCardCollector::default()),
        Box::new(FilesByCommitsCollector::default()),
        Box::new(FilesByLines::default()),
        Box::new(CommitsByFileExtension::default()),
        Box::new(UserSummaryCollector::default())
    ];
    stats_functions
}