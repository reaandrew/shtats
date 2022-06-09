use std::collections::HashMap;
use serde_json::{Error, json, Number};
use crate::models::GitCommit;
use crate::stats::{GitStat, JsonValue};
use crate::viewmodel::GitStatsJsonViewModelItem;

pub struct CommitsByFileExtension {
    data: HashMap<String, i32>,
}

impl CommitsByFileExtension{
    pub fn default() -> Self{
        return Self{
            data: Default::default()
        }
    }
}

impl JsonValue for CommitsByFileExtension {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let items = serde_json::Value::Array(self.data.iter().map(|(x, y)| {
            let name = serde_json::Value::String(String::from(x));
            let value = serde_json::Value::Number(Number::from(*y));
            return json!({
                "name": name,
                "value": value
            });
        }).collect::<Vec<serde_json::Value>>());
        return Ok(GitStatsJsonViewModelItem {
            summary: vec![],
            key: String::from("files_by_extension"),
            data: serde_json::to_value(items).unwrap(),
        });
    }
}

impl GitStat for CommitsByFileExtension {
    fn process(&mut self, commit: &GitCommit) {
        for operation in commit.clone().file_operations {
            let stat = self.data.entry(operation.file_extension)
                .or_insert(0);
            *stat += 1;
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::collectors::commits_by_file_extension::CommitsByFileExtension;
    use crate::models::{FileOperation, GitCommit, Operation};
    use crate::stats::{GitStat, JsonValue};

    #[test]
    fn test_process(){
        let mut subject = CommitsByFileExtension::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.file_operations.push(FileOperation{
            op: Operation::Added,
            file: "anything.rs".to_string(),
            file_extension: ".rs".to_string()
        });
        subject.process(&commit);

        assert_eq!(subject.data.len(), 1)
    }

    #[test]
    fn test_json_viewmodel(){
        let mut subject = CommitsByFileExtension::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.file_operations.push(FileOperation{
            op: Operation::Added,
            file: "anything.rs".to_string(),
            file_extension: ".rs".to_string()
        });
        subject.process(&commit);

        let result = subject.get_json_viewmodel().unwrap();
        assert_eq!(result.data.to_string(), "[{\"name\":\".rs\",\"value\":1}]");
    }
}