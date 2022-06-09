use std::collections::HashMap;
use serde_json::Error;
use crate::models::GitCommit;
use crate::stats::{GitStat, JsonValue, LineStats};
use crate::viewmodel::{GitStatsJsonViewModelItem, SummaryViewModelItem};

const HIGHEST_LINES_ADDED_NAME: &str = "Highest number of lines added for a single commit";
const HIGHEST_LINES_DELETED_NAME: &str = "Highest number of lines deleted for a single commit";
const HIGHEST_CHURN_NAME: &str = "Highest churn for a single commit (lines added + lines deleted)";

pub struct FilesByLines{
    most_lines_added_single_commit: i32,
    most_lines_deleted_single_commit: i32,
    most_churn_single_commit: i32,
    data: HashMap<String, LineStats>,
}

impl FilesByLines{
    pub fn default() -> Self {
        Self {
            most_lines_added_single_commit: 0,
            most_lines_deleted_single_commit: 0,
            most_churn_single_commit: 0,
            data: Default::default()
        }
    }
}

impl JsonValue for FilesByLines {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        Ok(GitStatsJsonViewModelItem {
            summary: vec![
                SummaryViewModelItem {
                    name: HIGHEST_LINES_ADDED_NAME.to_string(),
                    value: self.most_lines_added_single_commit.to_string(),
                },
                SummaryViewModelItem {
                    name: HIGHEST_LINES_DELETED_NAME.to_string(),
                    value: self.most_lines_deleted_single_commit.to_string(),
                },
                SummaryViewModelItem {
                    name: HIGHEST_CHURN_NAME.to_string(),
                    value: self.most_churn_single_commit.to_string(),
                },
            ],
            key: "files_by_commits".to_string(),
            data: Default::default(),
        })
    }
}

impl GitStat for FilesByLines{
    fn process(&mut self, commit: &GitCommit) {

        for operation in commit.clone().line_stats {
            if operation.lines_added > self.most_lines_added_single_commit{
                self.most_lines_added_single_commit = operation.lines_added;
            }

            if operation.lines_deleted > self.most_lines_deleted_single_commit{
                self.most_lines_deleted_single_commit = operation.lines_deleted;
            }

            let churn= operation.lines_added + operation.lines_deleted;
            if(churn) > self.most_churn_single_commit {
                self.most_churn_single_commit = churn;
            }

            let stat = self.data.entry(operation.file)
                .or_insert(LineStats {
                    added: 0,
                    deleted: 0,
                });
            stat.added += operation.lines_added;
            stat.deleted += operation.lines_deleted;
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::{GitCommit, GitStat};
    use crate::collectors::files_by_lines::FilesByLines;
    use crate::models::{GitCommit, LineStat};
    use crate::stats::{GitStat, JsonValue};

    #[test]
    fn test_process(){
        let mut subject = FilesByLines::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.line_stats.push(LineStat{
            lines_added: 1,
            lines_deleted: 2,
            file: "file1.rs".to_string()
        });
        commit.line_stats.push(LineStat{
            lines_added: 2,
            lines_deleted: 4,
            file: "file2.rs".to_string()
        });
        subject.process(&commit);

        assert_eq!(subject.most_churn_single_commit, 6);
        assert_eq!(subject.most_lines_added_single_commit, 2);
        assert_eq!(subject.most_lines_deleted_single_commit, 4);
    }

    #[test]
    fn test_json_viewmodel(){
        let mut subject = FilesByLines::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.line_stats.push(LineStat{
            lines_added: 1,
            lines_deleted: 2,
            file: "file1.rs".to_string()
        });
        commit.line_stats.push(LineStat{
            lines_added: 2,
            lines_deleted: 4,
            file: "file2.rs".to_string()
        });
        subject.process(&commit);

        let result = subject.get_json_viewmodel().unwrap();
        assert_eq!(result.summary.len(), 3);
    }
}