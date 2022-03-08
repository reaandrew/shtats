use std::collections::HashMap;
use serde_json::{Error, Number};
use crate::{GitCommit, GitStat};
use crate::stats::JsonValue;
use crate::viewmodel::{GitStatsJsonViewModelItem, KeyValue};

pub struct CommitsByDayCollector {
    total_commits_by_day: HashMap<String, i32>,
}

impl CommitsByDayCollector {
    pub fn default() -> Self {
        Self {
            total_commits_by_day: Default::default()
        }
    }
}

impl JsonValue for CommitsByDayCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let mut total_commits_by_day: Vec<KeyValue> = Default::default();
        for (key, value) in self.total_commits_by_day.clone() {
            total_commits_by_day.push(KeyValue {
                key,
                value,
            })
        }
        total_commits_by_day.sort_by(|a, b| a.key.cmp(&b.key));

        let items = total_commits_by_day.iter().map(|x| {
            return serde_json::Value::Array(vec![
                serde_json::Value::String(String::from(&x.key)),
                serde_json::Value::Number(Number::from(x.value)),
            ]);
        }).collect::<Vec<serde_json::Value>>();

        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("total_commits_by_day"),
            data: serde_json::to_value(items).unwrap(),
        });
    }
}

impl GitStat for CommitsByDayCollector {
    fn process(&mut self, commit: &GitCommit) {
        let stat = self.total_commits_by_day.entry(commit.day_key())
            .or_insert(0);
        *stat += 1;
    }
}