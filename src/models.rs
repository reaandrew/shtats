use std::str::FromStr;
use chrono::{DateTime, FixedOffset};
#[cfg(test)]
use std::fmt::{Display, Formatter};
#[cfg(test)]
use mockall::Any;
#[cfg(test)]
use serde_json::{json, Value};

#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Operation {
    Added = b'A',
    Modified = b'M',
    Deleted = b'D',
    Renamed = b'R',
    Copied = b'C',
    TypeChanged = b'T',
    Unmerged = b'U',
    Unknown = b'X',
    PairingBroken = b'B'
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, ()> {
        match s.chars().collect::<Vec<char>>().as_slice() {
            ['A'] => Ok(Operation::Added),
            ['M'] => Ok(Operation::Modified),
            ['D'] => Ok(Operation::Deleted),
            ['R', ..] => Ok(Operation::Renamed),
            ['C'] => Ok(Operation::Copied),
            ['T'] => Ok(Operation::TypeChanged),
            ['U'] => Ok(Operation::Unmerged),
            ['X'] => Ok(Operation::Unknown),
            ['B'] => Ok(Operation::PairingBroken),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct LineStat {
    pub(crate) lines_added: i32,
    pub(crate) lines_deleted: i32,
    pub(crate) file: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct FileOperation {
    pub(crate) op: Operation,
    pub(crate) file: String,
    pub(crate) file_extension: String,
}

#[derive(Clone, PartialEq)]
pub struct GitCommit {
    pub(crate) commit_hash: String,
    pub(crate) tags: Vec<String>,
    pub(crate) author: GitAuthor,
    pub(crate) date: DateTime<FixedOffset>,
    pub(crate) message: Vec<String>,
    pub(crate) file_operations: Vec<FileOperation>,
    pub(crate) line_stats: Vec<LineStat>,
}

impl Default for GitCommit{
    fn default() -> Self {
        return Self{
            commit_hash: "".to_string(),
            tags: vec![],
            author: Default::default(),
            date: DateTime::parse_from_rfc2822("Thu, 1 Jan 1970 01:01:01 +0000").unwrap(),
            message: vec![],
            file_operations: vec![],
            line_stats: vec![]
        }
    }
}

#[cfg(test)]
impl Display for GitCommit{

    #[cfg(test)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let commit_hash = serde_json::Value::from(self.commit_hash.clone());
        let tags = serde_json::Value::from(self.tags.clone());
        let author = json!({
            "name": self.author.name,
            "email": self.author.email
        });
        let date = serde_json::Value::from(self.date.to_rfc2822());
        let message = serde_json::Value::from(self.message.join("\n"));
        let file_operations = serde_json::Value::from(self.file_operations.iter().map(|x|{
            let file_extension = x.file_extension.clone();
            let file = x.file.clone();
            let op = x.op.type_name();
            json!({
                "extension":file_extension,
                "file": file,
                "op": op
            })
        }).collect::<Vec<Value>>());
        let line_stats = serde_json::Value::from(self.line_stats.iter().map(|x|{
            let file = x.file.clone();
            let added = x.lines_added;
            let deleted = x.lines_deleted;
            json!({
                "file": file,
                "lines_added": added,
                "lines_deleted": deleted
            })
        }).collect::<Vec<Value>>());
        let data = json!({
            "commit_hash": commit_hash,
            "tags": tags,
            "author": author,
            "date": date,
            "message": message,
            "file_operations": file_operations,
            "line_stats": line_stats
        });
        f.write_str(data.to_string().as_str())
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct GitAuthor{
    pub(crate) name: String,
    pub(crate) email: String
}

impl GitAuthor{
    pub(crate) fn key(&self) -> String{
        return format!("{} <{}>", self.name, self.email);
    }
}

impl GitCommit {
    pub(crate) fn day_key(&self) -> String {
        return self.date.format("%Y-%m-%d").to_string();
    }

    pub(crate) fn hour_key_by_weekday(&self) -> String {
        return self.date.format("%w-%H").to_string();
    }

    pub(crate) fn total_lines_added(&self) -> i32 {
        return self.line_stats.iter().map(|x| x.lines_added).sum();
    }

    pub(crate) fn total_lines_deleted(&self) -> i32 {
        return self.line_stats.iter().map(|x| x.lines_deleted).sum();
    }

    pub(crate) fn total_message_size(&self) -> i32 {
        return self.message.iter().map(|x| x.as_bytes().len() as i32).sum();
    }

    pub(crate) fn total_message_lines(&self) -> i32 {
        return self.message.len() as i32;
    }

    pub(crate) fn total_files_of_operation(&self, operation: Operation) -> i32{
        return self.file_operations.iter().filter(|x|x.op == operation).count() as i32;
    }
}


#[cfg(test)]
mod commit_tests {
    use crate::GitCommit;
    use crate::models::LineStat;

    #[test]
    fn test_commit_total_lines_added() {
        let mut commit = GitCommit::default();
        commit.line_stats.push(LineStat {
            lines_added: 1,
            lines_deleted: 2,
            file: "".to_string()
        });
        commit.line_stats.push(LineStat {
            lines_added: 4,
            lines_deleted: 6,
            file: "".to_string()
        });
        assert_eq!(5, commit.total_lines_added());
        assert_eq!(8, commit.total_lines_deleted());
    }
}
