use serde_json::Error;
use crate::models::GitCommit;
use crate::stats::{GitStat, JsonValue, SummaryStats};
use crate::viewmodel::{GitStatsJsonViewModelItem, SummaryViewModelItem};

#[derive(Clone)]
pub struct SummaryStatsCollector {
    pub(crate) summary: SummaryStats,
}

impl SummaryStatsCollector {
    pub fn default() -> Self {
        Self {
            summary: Default::default()
        }
    }
}

impl JsonValue for SummaryStatsCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let mut summary: Vec<SummaryViewModelItem> = Default::default();
        summary.push(SummaryViewModelItem {
            name: "First committer".to_string(),
            value: self.summary.first_committer.clone(),
        });
        summary.push(SummaryViewModelItem {
            name: "Date of first commit".to_string(),
            value: self.summary.date_first_commit.clone(),
        });
        summary.push(SummaryViewModelItem {
            name: "Number of commits_collection".to_string(),
            value: self.summary.commit_count.to_string(),
        });
        summary.push(SummaryViewModelItem {
            name: "Total lines_collection added".to_string(),
            value: self.summary.total_lines_added.to_string(),
        });
        summary.push(SummaryViewModelItem {
            name: "Total lines_collection deleted".to_string(),
            value: self.summary.total_lines_deleted.to_string(),
        });
        return Ok(GitStatsJsonViewModelItem {
            summary,
            key: String::from("summary"),
            data: Default::default(),
        });
    }
}

impl GitStat for SummaryStatsCollector {
    fn process(&mut self, commit: &GitCommit) {
        self.summary.commit_count += 1;

        if self.summary.date_first_commit.is_empty() {
            self.summary.date_first_commit = commit.date.to_string();
        }

        if self.summary.first_committer.is_empty() {
            self.summary.first_committer = commit.author.key();
        }

        self.summary.total_lines_added += commit.total_lines_added();
        self.summary.total_lines_deleted += commit.total_lines_deleted();
    }
}


#[cfg(test)]
mod summary_stats_collector_tests {
    use chrono::{DateTime, Duration, Utc};
    use crate::{GitCommit, GitStat};
    use crate::collectors::summary_stats::SummaryStatsCollector;
    use crate::models::{GitAuthor, GitCommit, LineStat};

    #[test]
    fn test_overall_commit_count_with_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.commit_hash = String::from("123");

        let mut collector = SummaryStatsCollector::default();

        collector.process(&commit);

        assert_eq!(1, collector.summary.commit_count);
    }

    #[test]
    fn test_date_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.date = DateTime::from(Utc::now());

        let mut collector = SummaryStatsCollector::default();

        collector.process(&commit);

        assert_eq!(commit.date.to_string(), collector.summary.date_first_commit);
    }

    #[test]
    fn test_date_2_commits() {
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.date = DateTime::from(Utc::now() - Duration::days(2));
        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.date = DateTime::from(Utc::now());

        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit_1);
        collector.process(&commit_2);
        assert_eq!(commit_1.date.to_string(), collector.summary.date_first_commit);
    }

    #[test]
    fn test_first_committer_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.author = GitAuthor { email: "email1".into(), name: "name1".into() };

        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit);

        assert_eq!(collector.summary.first_committer, "name1 <email1>");
    }

    #[test]
    fn test_first_committer_2_commits() {
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.author = GitAuthor { email: "email1".into(), name: "name1".into() };
        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.author = GitAuthor { email: "email2".into(), name: "name2".into() };

        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit_1);

        assert_eq!(collector.summary.first_committer, "name1 <email1>");
    }

    #[test]
    fn test_lines_added_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.line_stats = vec![LineStat {
            lines_added: 10,
            lines_deleted: 0,
            file: "".to_string(),
        }];

        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit);

        assert_eq!(collector.summary.total_lines_added, 10);
    }

    #[test]
    fn test_lines_added_2_commit() {
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.line_stats = vec![LineStat {
            lines_added: 10,
            lines_deleted: 0,
            file: "".to_string(),
        }];

        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.line_stats = vec![LineStat {
            lines_added: 5,
            lines_deleted: 0,
            file: "".to_string(),
        }];
        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit_1);
        collector.process(&commit_2);


        assert_eq!(collector.summary.total_lines_added, 15);
    }

    #[test]
    fn test_lines_deleted_1_commit() {
        let mut commit: GitCommit = GitCommit::default();
        commit.line_stats = vec![LineStat {
            lines_added: 0,
            lines_deleted: 2,
            file: "".to_string(),
        }];

        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit);

        assert_eq!(collector.summary.total_lines_deleted, 2);
    }

    #[test]
    fn test_lines_deleted_2_commit() {
        let mut commit_1: GitCommit = GitCommit::default();
        commit_1.line_stats = vec![LineStat {
            lines_added: 0,
            lines_deleted: 2,
            file: "".to_string(),
        }];

        let mut commit_2: GitCommit = GitCommit::default();
        commit_2.line_stats = vec![LineStat {
            lines_added: 0,
            lines_deleted: 7,
            file: "".to_string(),
        }];

        let mut collector = SummaryStatsCollector::default();
        collector.process(&commit_1);
        collector.process(&commit_2);

        assert_eq!(collector.summary.total_lines_deleted, 9);
    }
}