use crate::{GitCommit, GitStat, GitStats, LineStats};
use crate::stats::FileStats;

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


pub fn create_stat_collectors() -> Vec<Box<dyn GitStat>> {
    let stats_functions: Vec<Box<dyn GitStat>> = vec![
        Box::new(SummaryStatsCollector {}),
        Box::new(TotalCommitsByDayCollector {}),
        Box::new(TotalLinesByDayCollector {}),
        Box::new(MessageStatsCollector {}),
        Box::new(SimilarFilesChangingCollector{}),
        Box::new(TotalFilesByDayCollector{})
    ];
    stats_functions
}


#[cfg(test)]
mod collector_tests {
    use crate::{GitCommit, GitStat, GitStats};
    use crate::collectors::SummaryStatsCollector;
    use crate::process::process_commit;

    #[test]
    fn test_overall_commit_count_with_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.commit_hash = String::from("123");

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(SummaryStatsCollector {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats);

        assert_eq!(1, stats.summary.commit_count);
    }
}