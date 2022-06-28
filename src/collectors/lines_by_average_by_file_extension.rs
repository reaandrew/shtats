use std::collections::HashMap;
use serde_json::{Error, json};
use crate::models::{GitCommit};
use crate::stats::{GitStat, JsonValue, LineStats};
use crate::viewmodel::GitStatsJsonViewModelItem;

pub struct LinesByAverageByFileExtensionCollector{
    by_file_extension: HashMap<String, LineStats>,
    total_commits: i32
}

impl LinesByAverageByFileExtensionCollector {
    pub fn default() -> Self {
        Self {
            by_file_extension: Default::default(),
            total_commits: 0
        }
    }
}

impl JsonValue for LinesByAverageByFileExtensionCollector{
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let items = serde_json::Value::Array(self.by_file_extension.iter().map(|(x, y)| {
            return json!([x, y.added / self.total_commits as i64, y.deleted / self.total_commits as i64]);
        }).collect::<Vec<serde_json::Value>>());
        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("avg_line_stats_by_file_extension"),
            data: items,
        });
    }
}

impl GitStat for LinesByAverageByFileExtensionCollector {
    fn process(&mut self, commit: &GitCommit) {
        for line_stats in &commit.line_stats{
            let extension_stat = self.by_file_extension.entry(line_stats.extension())
                .or_insert(LineStats {
                    added: 0,
                    deleted: 0,
                });
            extension_stat.added += line_stats.lines_added as i64;
            extension_stat.deleted += line_stats.lines_deleted as i64;
        }

        self.total_commits += 1
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value};
    use crate::collectors::lines_by_average_by_file_extension::LinesByAverageByFileExtensionCollector;
    use crate::models::{GitCommit, GitCommitBuilder};
    use crate::stats::{GitStat, JsonValue, LineStats};

    struct KeyedLineStat{
        key: String,
        stats: LineStats
    }

    fn assert_line_stat(value: &Value, stat: KeyedLineStat){
        assert!(value.as_array().unwrap().iter().find(|&x| {
            let data = x.as_array().unwrap();
            return data.get(0).unwrap().as_str().unwrap() == stat.key &&
                data.get(1).unwrap().as_i64().unwrap() == stat.stats.added as i64 &&
                data.get(2).unwrap().as_i64().unwrap() == stat.stats.deleted as i64

        }).is_some());
    }

    #[test]
    fn test_process() {
        let mut subject = LinesByAverageByFileExtensionCollector::default();
        let commit: GitCommit = GitCommitBuilder::new()
            .with_lines(1, 2, "a.txt")
            .build();
        subject.process(&commit);

        assert_eq!(subject.by_file_extension.len(), 1)
    }

    #[test]
    fn test_json_viewmodel() {
        let mut subject = LinesByAverageByFileExtensionCollector::default();
        subject.process(&GitCommitBuilder::new()
            .for_date_time("2022-01-01 12:00:00")
            .with_lines(2, 12, "1.txt")
            .with_lines(4, 6, "1.js")
            .build());
        subject.process(&GitCommitBuilder::new()
            .for_date_time("2022-01-3 18:00:00")
            .with_lines(2, 12, "1.txt")
            .with_lines(4, 6, "1.js")
            .build());

        let result = subject.get_json_viewmodel().unwrap();
        //TODO: Create a struct for the result, deserialize the result and assert on the struct
        //      This applies to all types of assertions like this.
        assert_eq!(result.data.as_array().unwrap().len(), 2);

        assert_line_stat(&result.data, KeyedLineStat{
            key: "js".to_string(),
            stats: LineStats{
                added: 4,
                deleted: 6
            }
        });
        assert_line_stat(&result.data, KeyedLineStat{
            key: "txt".to_string(),
            stats: LineStats{
                added: 2,
                deleted: 12
            }
        });
    }
}