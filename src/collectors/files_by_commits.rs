use std::collections::HashMap;
use itertools::{max, min};
use serde_json::{Error};
use crate::{GitCommit, GitStat};
use crate::stats::JsonValue;
use crate::viewmodel::{GitStatsJsonViewModelItem, SummaryViewModelItem};

const LOWEST_COMMIT_NUMBER_NAME: &str = "Lowest number of commits for a file";
const HIGHEST_COMMIT_NUMBER_NAME: &str = "Highest number of commits for a file";

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

    fn get_lowest_commits(&self) -> &i32 {
        let lowest_commits = min(self.data.values()).unwrap();
        lowest_commits
    }

    fn count_files_with_commits(&self, count: &i32) -> usize {
        let files_with_lowest_commits = (self.data.clone()).into_iter()
            .filter(|(_path, commit_count)| commit_count == count).count();
        files_with_lowest_commits
    }

    fn get_highest_commits(&self) -> &i32 {
        let highest_commits = max(self.data.values()).unwrap();
        highest_commits
    }
}

impl JsonValue for FilesByCommitsCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let lowest_commits = self.get_lowest_commits();
        let files_with_lowest_commits = self.count_files_with_commits(lowest_commits);
        let highest_commits = self.get_highest_commits();
        let files_with_highest_commits =self.count_files_with_commits(highest_commits);

        // let object = json!({
        //     "lowest_number_of_commits": lowest_commits,
        //     "with_lowest_commits": lowest_items,
        //     "highest_number_of_commits": highest_commits,
        //     "with_highest_commits": highest_items
        // });

        let pluralize = |value:String, input: usize|->String{
            return if input > 1 {
                format!("{}s", value)
            } else {
                value
            }
        };

        Ok(GitStatsJsonViewModelItem {
            summary: vec![
                SummaryViewModelItem {
                    name: LOWEST_COMMIT_NUMBER_NAME.to_string(),
                    value: format!("{} ({} {})",
                                   lowest_commits.to_string(),
                                   files_with_lowest_commits,
                    pluralize(String::from("file"), files_with_lowest_commits)),
                },
                SummaryViewModelItem {
                    name: HIGHEST_COMMIT_NUMBER_NAME.to_string(),
                    value: format!("{} ({} {})",
                                   highest_commits.to_string(),
                                   files_with_highest_commits,
                                   pluralize(String::from("file"), files_with_highest_commits)),
                },
            ],
            key: "files_by_commits".to_string(),
            data: Default::default(),
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
    use crate::collectors::files_by_commits::{FilesByCommitsCollector, HIGHEST_COMMIT_NUMBER_NAME, LOWEST_COMMIT_NUMBER_NAME};
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
                FileOperation { op: Operation::Added, file: "file".to_string(), file_extension: "".to_string() }
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
            FileOperation { op: Operation::Added, file: "file1".to_string(), file_extension: "".to_string() },
            FileOperation { op: Operation::Added, file: "file2".to_string(), file_extension: "".to_string() },
        ];
        collector.process(&commit1);

        let mut commit2 = GitCommit::default();
        commit2.file_operations = vec![
            FileOperation { op: Operation::Modified, file: "file2".to_string(), file_extension: "".to_string() }
        ];
        collector.process(&commit2);

        let result = collector.get_json_viewmodel().unwrap();
        assert_eq!(2, result.summary.len());
        assert_eq!(LOWEST_COMMIT_NUMBER_NAME.to_string(), result.summary.get(0).unwrap().name);
        assert_eq!("1 (1 file)", result.summary.get(0).unwrap().value);
        assert_eq!(HIGHEST_COMMIT_NUMBER_NAME.to_string(), result.summary.get(1).unwrap().name);
        assert_eq!("2 (1 file)", result.summary.get(1).unwrap().value);
    }
}