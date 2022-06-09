use std::collections::HashMap;
use serde_json::{Error, Number};
use crate::models::GitCommit;
use crate::models::Operation::{Added, Copied, Deleted, Modified, PairingBroken, Renamed, TypeChanged, Unknown, Unmerged};
use crate::stats::{FileStats, GitStat, JsonValue};
use crate::viewmodel::{FilesValue, GitStatsJsonViewModelItem};

pub struct FilesByDayCollector {
    total_files_by_day: HashMap<String, FileStats>,
}

impl FilesByDayCollector {
    pub fn default() -> Self {
        Self {
            total_files_by_day: Default::default()
        }
    }
}

impl JsonValue for FilesByDayCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let mut total_files_by_day: Vec<FilesValue> = Default::default();
        for (key, value) in self.total_files_by_day.clone() {
            total_files_by_day.push(FilesValue {
                key,
                files_added: value.added,
                files_deleted: value.deleted,
                files_modified: value.modified,
                files_renamed: value.renamed,
                files_copied: value.copied,
                files_type_changed: value.type_changed,
                files_unmerged: value.unmerged,
                files_unknown: value.unknown,
                files_pairing_broken: value.pairing_broken
            })
        }
        total_files_by_day.sort_by(|a, b| a.key.cmp(&b.key));
        let items = total_files_by_day.iter().map(|x| {
            return serde_json::Value::Array(vec![
                serde_json::Value::String(String::from(&x.key)),
                serde_json::Value::Number(Number::from(x.files_added)),
                serde_json::Value::Number(Number::from(x.files_modified)),
                serde_json::Value::Number(Number::from(x.files_deleted)),
                serde_json::Value::Number(Number::from(x.files_renamed)),
                serde_json::Value::Number(Number::from(x.files_copied)),
                serde_json::Value::Number(Number::from(x.files_type_changed)),
                serde_json::Value::Number(Number::from(x.files_unmerged)),
                serde_json::Value::Number(Number::from(x.files_unknown)),
                serde_json::Value::Number(Number::from(x.files_pairing_broken)),
            ]);
        }).collect::<Vec<serde_json::Value>>();

        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("total_files_by_day"),
            data: serde_json::to_value(items).unwrap(),
        });
    }
}

impl GitStat for FilesByDayCollector {
    fn process(&mut self, commit: &GitCommit) {
        let stat = self.total_files_by_day.entry(commit.day_key())
            .or_insert(FileStats {
                added: 0,
                modified: 0,
                deleted: 0,
                renamed: 0,
                copied: 0,
                type_changed: 0,
                unmerged: 0,
                unknown: 0,
                pairing_broken: 0
            });
        stat.added += commit.total_files_of_operation(Added);
        stat.deleted += commit.total_files_of_operation(Deleted);
        stat.modified += commit.total_files_of_operation(Modified);
        stat.renamed += commit.total_files_of_operation(Renamed);
        stat.copied += commit.total_files_of_operation(Copied);
        stat.type_changed += commit.total_files_of_operation(TypeChanged);
        stat.unmerged += commit.total_files_of_operation(Unmerged);
        stat.unknown += commit.total_files_of_operation(Unknown);
        stat.pairing_broken += commit.total_files_of_operation(PairingBroken);
    }
}

#[cfg(test)]
mod tests{
    use crate::collectors::files_by_day::FilesByDayCollector;
    use crate::models::{FileOperation, GitCommit, Operation};
    use crate::stats::{GitStat, JsonValue};

    #[test]
    fn test_process(){
        let mut subject = FilesByDayCollector::default();
        let commit: GitCommit = GitCommit::default();
        subject.process(&commit);

        assert_eq!(subject.total_files_by_day.len(), 1)
    }

    #[test]
    fn test_json_viewmodel(){
        let mut subject = FilesByDayCollector::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.file_operations.push(FileOperation{
            op: Operation::Added,
            file: "anything.rs".to_string(),
            file_extension: ".rs".to_string()
        });
        subject.process(&commit);

        let result = subject.get_json_viewmodel().unwrap();
        assert_eq!(result.data.to_string(), "[[\"1970-01-01\",1,0,0,0,0,0,0,0,0]]");
    }
}