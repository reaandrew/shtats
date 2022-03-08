use std::collections::HashMap;
use serde_json::{Error, Number};
use crate::{GitCommit, GitStat};
use crate::stats::{FileStats, JsonValue};
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
            })
        }
        total_files_by_day.sort_by(|a, b| a.key.cmp(&b.key));
        let items = total_files_by_day.iter().map(|x| {
            return serde_json::Value::Array(vec![
                serde_json::Value::String(String::from(&x.key)),
                serde_json::Value::Number(Number::from(x.files_added)),
                serde_json::Value::Number(Number::from(x.files_deleted)),
                serde_json::Value::Number(Number::from(x.files_modified)),
                serde_json::Value::Number(Number::from(x.files_renamed)),
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
            });
        stat.added += commit.total_files_added();
        stat.deleted += commit.total_files_deleted();
        stat.modified += commit.total_files_modified();
        stat.renamed += commit.total_files_renamed();
    }
}