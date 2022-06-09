use std::collections::HashMap;
use chrono::{Datelike, Timelike};
use serde_json::{Error, Number};
use crate::models::GitCommit;
use crate::stats::{GitStat, JsonValue, PunchStats};
use crate::viewmodel::{GitStatsJsonViewModelItem, PunchesValue};

pub struct PunchCardCollector {
    punchcard: HashMap<String, PunchStats>,
}

impl PunchCardCollector {
    pub fn default() -> Self {
        Self {
            punchcard: HashMap::new()
        }
    }
}

impl JsonValue for PunchCardCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let mut punch_data = Vec::<PunchesValue>::new();
        for (_, value) in self.punchcard.clone() {
            punch_data.push(PunchesValue {
                weekday: value.weekday,
                hour: value.hour,
                commits: value.commits,
            })
        }
        let items = punch_data.iter().map(|x| {
            return serde_json::Value::Array(vec![
                serde_json::Value::Number(Number::from(x.weekday)),
                serde_json::Value::Number(Number::from(x.hour)),
                serde_json::Value::Number(Number::from(x.commits)),
            ]);
        }).collect::<Vec<serde_json::Value>>();

        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("punch_data"),
            data: serde_json::to_value(items).unwrap(),
        });
    }
}

impl GitStat for PunchCardCollector {
    fn process(&mut self, commit: &GitCommit) {
        let stat = self.punchcard.entry(commit.hour_key_by_weekday())
            .or_insert(PunchStats {
                weekday: commit.date.weekday().num_days_from_sunday(),
                hour: commit.date.hour(),
                commits: 0,
            });
        stat.commits += 1
    }
}

#[cfg(test)]
mod tests{
    use crate::collectors::punch_card::PunchCardCollector;
    use crate::models::GitCommit;
    use crate::stats::{GitStat, JsonValue};

    #[test]
    fn test_process(){
        let mut subject = PunchCardCollector::default();
        let commit: GitCommit = GitCommit::default();
        subject.process(&commit);

        assert_eq!(subject.punchcard.len(), 1)
    }

    #[test]
    fn test_json_viewmodel(){
        let mut subject = PunchCardCollector::default();
        let commit: GitCommit = GitCommit::default();
        subject.process(&commit);

        let result = subject.get_json_viewmodel().unwrap();
        assert_eq!(result.data.to_string(), "[[4,1,1]]");
    }
}