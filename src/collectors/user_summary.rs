use std::collections::HashMap;
use itertools::Itertools;
use serde_json::{Error, json, Number};
use crate::{GitCommit, GitStat};
use crate::stats::JsonValue;
use crate::viewmodel::{GitStatsJsonViewModelItem, KeyValue};

pub struct UserStats {
    total_commits_by_day: HashMap<String, i32>,
    total_lines_added: i32,
    total_lines_deleted: i32,
    total_commits: i32,
    name: String,
    gravatar: String,
}

pub struct UserSummaryCollector {
    summary_by_user: HashMap<String, UserStats>,
}

impl UserSummaryCollector {
    pub fn default() -> Self {
        Self {
            summary_by_user: Default::default()
        }
    }
}

impl JsonValue for UserSummaryCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {

        let data = self.summary_by_user.values().clone().into_iter()
            .sorted_by(|a,b|Ord::cmp(&b.total_commits, &a.total_commits))
            .take(25)
            .collect::<Vec<&UserStats>>();

        let items = data.iter().enumerate().map(|(index, &stat)| {
            let mut total_commits_by_day: Vec<KeyValue> = Default::default();
            for (key, value) in stat.total_commits_by_day.clone() {
                total_commits_by_day.push(KeyValue {
                    key,
                    value,
                })
            }
            total_commits_by_day.sort_by(|a, b| a.key.cmp(&b.key));

            let commits  = total_commits_by_day.iter().map(|x| {
                return serde_json::Value::Array(vec![
                    serde_json::Value::String(String::from(&x.key)),
                    serde_json::Value::Number(Number::from(x.value)),
                ]);
            }).collect::<Vec<serde_json::Value>>();

            return json!({
                "commits": commits,
                "lines_added": stat.total_lines_added,
                "lines_deleted": stat.total_lines_deleted,
                "total_commits": stat.total_commits,
                "name": stat.name,
                "index": index+1,
                "gravatar": stat.gravatar
            });
        }).collect::<Vec<serde_json::Value>>();

        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("user_summary_stats"),
            data:  serde_json::to_value(items).unwrap(),
        });
    }
}

impl GitStat for UserSummaryCollector {
    fn process(&mut self, commit: &GitCommit) {
        let stat = self.summary_by_user.entry(commit.author.key())
            .or_insert(UserStats {
                total_commits_by_day: Default::default(),
                total_lines_added: 0,
                total_lines_deleted: 0,
                total_commits: 0,
                name: commit.author.name.clone(),
                gravatar: format!("{:x}",md5::compute(commit.author.email.clone())),
            });
        let commits_stat = stat.total_commits_by_day.entry(commit.day_key())
            .or_insert(0);
        *commits_stat += 1;
        stat.total_lines_added += commit.total_lines_added();
        stat.total_lines_deleted += commit.total_lines_deleted();
        stat.total_commits += 1;
    }
}