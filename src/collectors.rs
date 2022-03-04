use chrono::{Datelike, Timelike};
use crate::{GitCommit, GitStat, GitStats, LineStats};
use crate::stats::{FileStats, PunchStats};

struct SummaryStatsCollector {}

impl GitStat for SummaryStatsCollector {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        stats.summary.commit_count += 1;

        if stats.summary.date_first_commit.is_empty() {
            stats.summary.date_first_commit = commit.date.to_string();
        }

        if stats.summary.first_committer.is_empty() {
            stats.summary.first_committer = String::from(&commit.author);
        }

        stats.summary.total_lines_added += commit.total_lines_added();
        stats.summary.total_lines_deleted += commit.total_lines_deleted();
    }
}

struct TotalCommitsByDayCollector {}

impl GitStat for TotalCommitsByDayCollector {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        let stat = stats.total_commits_by_day.entry(commit.day_key())
            .or_insert(0);
        *stat += 1;
    }
}

struct TotalLinesByDayCollector {}

impl GitStat for TotalLinesByDayCollector {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        let stat = stats.total_lines_by_day.entry(commit.day_key())
            .or_insert(LineStats {
                added: 0,
                deleted: 0,
            });
        stat.added += commit.total_lines_added();
        stat.deleted += commit.total_lines_deleted();
    }
}

struct TotalFilesByDayCollector{}

impl GitStat for TotalFilesByDayCollector{
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        let stat = stats.total_files_by_day.entry(commit.day_key())
            .or_insert(FileStats {
                added: 0,
                modified: 0,
                deleted: 0,
                renamed: 0
            });
        stat.added += commit.total_files_added();
        stat.deleted += commit.total_files_deleted();
        stat.modified += commit.total_files_modified();
        stat.renamed += commit.total_files_renamed();
    }
}

struct MessageStatsCollector {}

impl GitStat for MessageStatsCollector {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        stats.total_message_lines += commit.total_message_lines();
        stats.total_message_size += commit.total_message_size();

        if commit.total_message_size() > stats.message_stats.max_size {
            stats.message_stats.max_size = commit.total_message_size()
        }

        if commit.total_message_lines() > stats.message_stats.max_lines {
            stats.message_stats.max_lines = commit.total_message_lines();
        }

        if commit.total_message_size() <= stats.message_stats.min_size {
            stats.message_stats.min_size = commit.total_message_size()
        }

        if commit.total_message_lines() <= stats.message_stats.min_lines {
            stats.message_stats.min_lines = commit.total_message_lines();
        }

        stats.message_stats.avg_size = stats.total_message_size / stats.summary.commit_count;
        stats.message_stats.avg_lines = stats.total_message_lines / stats.summary.commit_count
    }
}

struct SimilarFilesChangingCollector{

}

impl GitStat for SimilarFilesChangingCollector{
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        let files = commit.file_operations
            .iter()
            .map(|x|x.file.as_str())
            .collect::<Vec<&str>>();
        stats.dup_detector.add(files);
    }
}

struct PunchCardCollector{

}

impl GitStat for PunchCardCollector{
    fn process(&self, commit: &GitCommit, stats: &mut GitStats) {
        let stat = stats.punchcard.entry(commit.hour_key_by_weekday())
            .or_insert(PunchStats {
                weekday: commit.date.weekday().num_days_from_sunday(),
                hour: commit.date.hour(),
                commits: 0
            });
        stat.commits += 1
    }
}

pub fn create_stat_collectors() -> Vec<Box<dyn GitStat>> {
    let stats_functions: Vec<Box<dyn GitStat>> = vec![
        Box::new(SummaryStatsCollector {}),
        Box::new(TotalCommitsByDayCollector {}),
        Box::new(TotalLinesByDayCollector {}),
        Box::new(MessageStatsCollector {}),
        //Box::new(SimilarFilesChangingCollector{}),
        Box::new(TotalFilesByDayCollector{}),
        Box::new(PunchCardCollector{})
    ];
    stats_functions
}


#[cfg(test)]
mod summary_stats_collector_tests {
    use chrono::{DateTime, Duration, Utc};
    use crate::{GitCommit, GitStat, GitStats};
    use crate::collectors::SummaryStatsCollector;
    use crate::models::LineStat;
    use crate::process::process_commit;

    #[test]
    fn test_overall_commit_count_with_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.commit_hash = String::from("123");

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats, &||{});

        assert_eq!(1, stats.summary.commit_count);
    }

    #[test]
    fn test_date_1_commit(){
        let mut commit: GitCommit = GitCommit::default();
        commit.date = DateTime::from(Utc::now());

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats, &||{});

        assert_eq!(commit.date.to_string(), stats.summary.date_first_commit);
    }

    #[test]
    fn test_date_2_commits(){
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.date = DateTime::from(Utc::now() - Duration::days(2));
        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.date = DateTime::from(Utc::now());

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit_1, &stat_functions, &mut stats, &||{});
        process_commit(&commit_2, &stat_functions, &mut stats, &||{});

        assert_eq!(commit_1.date.to_string(), stats.summary.date_first_commit);
    }

    #[test]
    fn test_first_committer_1_commit(){
        let mut commit: GitCommit = GitCommit::default();
        commit.author = String::from("Bob");

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats, &||{});

        assert_eq!(stats.summary.first_committer, "Bob");
    }

    #[test]
    fn test_first_committer_2_commits(){
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.author = String::from("Jeff");
        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.author = String::from("Alan");

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit_1, &stat_functions, &mut stats, &||{});
        process_commit(&commit_2, &stat_functions, &mut stats, &||{});

        assert_eq!(stats.summary.first_committer, "Jeff");
    }

    #[test]
    fn test_lines_added_1_commit(){
        let mut commit: GitCommit = GitCommit::default();
        commit.line_stats = vec![LineStat{
            lines_added: 10,
            lines_deleted: 0
        }];

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats, &||{});

        assert_eq!(stats.summary.total_lines_added, 10);
    }

    #[test]
    fn test_lines_added_2_commit(){
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.line_stats = vec![LineStat{
            lines_added: 10,
            lines_deleted: 0
        }];

        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.line_stats = vec![LineStat{
            lines_added: 5,
            lines_deleted: 0
        }];

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit_1, &stat_functions, &mut stats, &||{});
        process_commit(&commit_2, &stat_functions, &mut stats, &||{});


        assert_eq!(stats.summary.total_lines_added, 15);
    }

    #[test]
    fn test_lines_deleted_1_commit(){
        let mut commit: GitCommit = GitCommit::default();
        commit.line_stats = vec![LineStat{
            lines_added: 0,
            lines_deleted: 2
        }];

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats, &||{});

        assert_eq!(stats.summary.total_lines_deleted, 2);
    }

    #[test]
    fn test_lines_deleted_2_commit(){
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.line_stats = vec![LineStat{
            lines_added: 0,
            lines_deleted: 2
        }];

        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.line_stats = vec![LineStat{
            lines_added: 0,
            lines_deleted: 7
        }];

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit_1, &stat_functions, &mut stats, &||{});
        process_commit(&commit_2, &stat_functions, &mut stats, &||{});


        assert_eq!(stats.summary.total_lines_deleted, 9 );
    }
}