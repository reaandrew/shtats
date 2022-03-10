use std::collections::HashMap;
use itertools::{min};
use serde_json::{Error};
use crate::{GitCommit, GitStat};
use crate::stats::JsonValue;
use crate::viewmodel::{GitStatsJsonViewModelItem, SummaryViewModelItem};

const LOWEST_COMMIT_NUMBER_NAME: &str = "Lowest number of commits for a file";
//const HIGHEST_COMMIT_NUMBER_NAME: &str = "Highest number of commits for a file";

pub struct FilesByCommitsCollector {
    total_commits: i32,
    pub data: HashMap<String, i32>,
}


impl FilesByCommitsCollector {
    pub fn default() -> Self {
        Self {
            total_commits: 0,
            data: Default::default(),
        }
    }
}

impl JsonValue for FilesByCommitsCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let lowest_commits = min(self.data.values()).unwrap();
        //let highest_commits = max(self.data.values()).unwrap();

        Ok(GitStatsJsonViewModelItem {
            summary: vec![
                SummaryViewModelItem { name: LOWEST_COMMIT_NUMBER_NAME.to_string(), value: lowest_commits.to_string() },
                //SummaryViewModelItem { name: HIGHEST_COMMIT_NUMBER_NAME.to_string(), value: highest_commits.to_string() }
            ],
            key: "files_by_commits".to_string(),
            data: Default::default()
        })
    }
}

impl GitStat for FilesByCommitsCollector {
    fn process(&mut self, commit: &GitCommit) {
        self.total_commits += 1;

        for operation in commit.clone().file_operations {
            let stat = self.data.entry(operation.file)
                .or_insert(0);
            *stat += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collectors::files_by_commits::{FilesByCommitsCollector, LOWEST_COMMIT_NUMBER_NAME};
    use crate::{GitCommit, GitStat};
    use crate::models::{FileOperation, Operation};
    use crate::stats::JsonValue;

    #[test]
    fn sums_commits_per_file() {
        let mut collector = FilesByCommitsCollector::default();
        let commits = vec![
            GitCommit::default(),
            GitCommit::default(),
            GitCommit::default(),
        ];
        for mut commit in commits {
            commit.file_operations = vec![
                FileOperation { op: Operation::ADD, file: "file".to_string() }
            ];
            collector.process(&commit);
        }

        assert_eq!(3, *collector.data.get("file").unwrap());
    }

    #[test]
    fn json_view_has_lowest_number_of_commits() {
        let mut collector = FilesByCommitsCollector::default();
        let mut commit1 = GitCommit::default();
        commit1.file_operations = vec![
            FileOperation { op: Operation::ADD, file: "file1".to_string() },
            FileOperation { op: Operation::ADD, file: "file2".to_string() },
        ];
        collector.process(&commit1);

        let mut commit2 = GitCommit::default();
        commit2.file_operations = vec![
            FileOperation { op: Operation::MODIFY, file: "file2".to_string() }
        ];
        collector.process(&commit2);

        let result = collector.get_json_viewmodel().unwrap();
        assert_eq!(1, result.summary.len());
        assert_eq!(LOWEST_COMMIT_NUMBER_NAME.to_string(), result.summary.get(0).unwrap().name);
        assert_eq!("1", result.summary.get(0).unwrap().value);
    }
}