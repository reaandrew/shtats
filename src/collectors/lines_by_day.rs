use std::collections::HashMap;
use serde_json::{Error, Number};
use crate::{GitCommit, GitStat, LineStats};
use crate::stats::JsonValue;
use crate::viewmodel::{GitStatsJsonViewModelItem, LinesValue};

pub struct LinesByDayCollector {
    total_lines_by_day: HashMap<String, LineStats>,
}

impl LinesByDayCollector {
    pub fn default() -> Self {
        Self {
            total_lines_by_day: Default::default()
        }
    }
}

impl JsonValue for LinesByDayCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let mut total_lines_by_day: Vec<LinesValue> = Default::default();
        for (key, value) in self.total_lines_by_day.clone() {
            total_lines_by_day.push(LinesValue {
                key,
                lines_added: value.added,
                lines_deleted: value.deleted,
            })
        }
        total_lines_by_day.sort_by(|a, b| a.key.cmp(&b.key));

        let items = total_lines_by_day.iter().map(|x| {
            return serde_json::Value::Array(vec![
                serde_json::Value::String(String::from(&x.key)),
                serde_json::Value::Number(Number::from(x.lines_added)),
                serde_json::Value::Number(Number::from(x.lines_deleted)),
            ]);
        }).collect::<Vec<serde_json::Value>>();

        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("total_lines_by_day"),
            data: serde_json::to_value(items).unwrap(),
        });
    }
}

impl GitStat for LinesByDayCollector {
    fn process(&mut self, commit: &GitCommit) {
        let stat = self.total_lines_by_day.entry(commit.day_key())
            .or_insert(LineStats {
                added: 0,
                deleted: 0,
            });
        stat.added += commit.total_lines_added();
        stat.deleted += commit.total_lines_deleted();
    }
}

#[cfg(test)]
mod tests{
    use crate::{GitCommit, GitStat};
    use crate::collectors::lines_by_day::LinesByDayCollector;
    use crate::models::LineStat;
    use crate::stats::JsonValue;

    #[test]
    fn test_process(){
        let mut subject = LinesByDayCollector::default();
        let commit: GitCommit = GitCommit::default();
        subject.process(&commit);

        assert_eq!(subject.total_lines_by_day.len(), 1)
    }

    #[test]
    fn test_json_viewmodel(){
        let mut subject = LinesByDayCollector::default();
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
        assert_eq!(result.data.to_string(), "[[\"1970-01-01\",3,6]]");
    }
}