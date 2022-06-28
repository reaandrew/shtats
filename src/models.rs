use std::ffi::OsStr;
use std::str::FromStr;
use chrono::{Datelike, DateTime, FixedOffset};
#[cfg(test)]
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::path::Path;
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
    PairingBroken = b'B',
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

impl LineStat{
    pub(crate) fn extension(&self) -> String{
        let extension = String::from(Path::new(self.file.as_str())
            .extension()
            .and_then(OsStr::to_str).unwrap_or(""));
        return extension;
    }
}

#[cfg(test)]
mod line_stat_tests{
    use crate::models::LineStat;

    #[test]
    fn test_extension(){
        let subject = LineStat{
            lines_added: 0,
            lines_deleted: 0,
            file: "something.rs".to_string()
        };
        assert_eq!("rs", subject.extension());
    }
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

impl Default for GitCommit {
    fn default() -> Self {
        return Self {
            commit_hash: "".to_string(),
            tags: vec![],
            author: Default::default(),
            date: DateTime::parse_from_rfc2822("Thu, 1 Jan 1970 01:01:01 +0000").unwrap(),
            message: vec![],
            file_operations: vec![],
            line_stats: vec![],
        };
    }
}


#[cfg(test)]
impl Display for GitCommit {
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
        let file_operations = serde_json::Value::from(self.file_operations.iter().map(|x| {
            let file_extension = x.file_extension.clone();
            let file = x.file.clone();
            let op = x.op.type_name();
            json!({
                "extension":file_extension,
                "file": file,
                "op": op
            })
        }).collect::<Vec<Value>>());
        let line_stats = serde_json::Value::from(self.line_stats.iter().map(|x| {
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
pub struct GitAuthor {
    pub(crate) name: String,
    pub(crate) email: String,
}

impl GitAuthor {
    pub(crate) fn key(&self) -> String {
        return format!("{} <{}>", self.name, self.email);
    }
}

impl GitCommit {
    pub(crate) fn day_key(&self) -> String {
        return self.date.format("%Y-%m-%d").to_string();
    }

    pub(crate) fn month_key(&self) -> String {
        return self.date.format("%Y-%m").to_string();
    }

    pub(crate) fn hour_key(&self) -> String{
        return self.date.format("%Y-%m-%d %H").to_string();
    }

    pub(crate) fn week_key(&self) -> String{
        let week = self.date.iso_week().week();
        return self.date.format("%Y ").to_string().add(week.to_string().as_str())
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

    pub(crate) fn total_files_of_operation(&self, operation: Operation) -> i32 {
        return self.file_operations.iter().filter(|x| x.op == operation).count() as i32;
    }
}


#[cfg(test)]
mod commit_tests {
    use crate::models::{GitCommit, LineStat};

    #[test]
    fn test_commit_total_lines_added() {
        let mut commit = GitCommit::default();
        commit.line_stats.push(LineStat {
            lines_added: 1,
            lines_deleted: 2,
            file: "".to_string(),
        });
        commit.line_stats.push(LineStat {
            lines_added: 4,
            lines_deleted: 6,
            file: "".to_string(),
        });
        assert_eq!(5, commit.total_lines_added());
        assert_eq!(8, commit.total_lines_deleted());
    }
}

#[cfg(test)]
pub struct GitCommitBuilder{
    commit: GitCommit
}

#[cfg(test)]
impl GitCommitBuilder{
    pub(crate) fn new() -> GitCommitBuilder{
        return GitCommitBuilder{ commit: Default::default() }
    }

    pub(crate) fn with_lines(mut self, added:i32, deleted: i32, file: &str) -> GitCommitBuilder{
        self.commit.line_stats.push(LineStat{
            lines_added: added,
            lines_deleted: deleted,
            file: file.to_string()
        });
        return self;
    }


    pub(crate) fn for_date(mut self, date: &str)-> GitCommitBuilder{
        let date_value = date.to_owned() + "00:00:00 +00:00";
        self.commit.date = DateTime::parse_from_str(date_value.as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap();
        return self;
    }

    pub(crate) fn for_date_time(mut self, date: &str)-> GitCommitBuilder{
        let date_value = date.to_owned() + " +00:00";
        self.commit.date = DateTime::parse_from_str(date_value.as_str(), "%Y-%m-%d %H:%M:%S %z").unwrap();
        return self;
    }

    pub(crate) fn build(self) -> GitCommit{
        return self.commit.clone();
    }
}

#[cfg(test)]
mod commit_builder_tests{
    use chrono::DateTime;
    use crate::models::GitCommitBuilder;

    #[test]
    fn test_with_lines(){
        let commit = GitCommitBuilder::new()
            .with_lines(1,2,"something.rs")
            .build();
        assert_eq!(commit.line_stats.len(), 1);
        assert_eq!(commit.line_stats[0].lines_added, 1);
        assert_eq!(commit.line_stats[0].lines_deleted,2);
        assert_eq!(commit.line_stats[0].file, "something.rs");
    }

    #[test]
    fn test_for_date(){
        let date_value = "2022-01-01 00:00:00 +00:00";
        let date = DateTime::parse_from_str(date_value, "%Y-%m-%d %H:%M:%S %z").unwrap();
        //assert_eq!(DateTime::parse_from_rfc2822("Wed, 2 Feb 2022 12:02:17 +0000").unwrap(), actual);

        let commit = GitCommitBuilder::new()
            .for_date("2022-01-01")
            .build();

        assert_eq!(commit.date, date);
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::models::Operation;

    #[test]
    fn operations_added_from_string() {
        assert_eq!(Operation::Added, Operation::from_str("A").unwrap());
    }

    #[test]
    fn operations_copied_from_string() {
        assert_eq!(Operation::Copied, Operation::from_str("C").unwrap());
    }

    #[test]
    fn operations_deleted_from_string() {
        assert_eq!(Operation::Deleted, Operation::from_str("D").unwrap());
    }

    #[test]
    fn operations_modified_from_string() {
        assert_eq!(Operation::Modified, Operation::from_str("M").unwrap());
    }

    #[test]
    fn operations_pairing_broken_from_string() {
        assert_eq!(Operation::PairingBroken, Operation::from_str("B").unwrap());
    }

    #[test]
    fn operations_renamed_from_string() {
        assert_eq!(Operation::Renamed, Operation::from_str("R").unwrap());
    }

    #[test]
    fn operations_unknown_from_string() {
        assert_eq!(Operation::Unknown, Operation::from_str("X").unwrap());
    }

    #[test]
    fn operations_unmerged_from_string() {
        assert_eq!(Operation::Unmerged, Operation::from_str("U").unwrap());
    }

    #[test]
    fn operations_type_changed_from_string() {
        assert_eq!(Operation::TypeChanged, Operation::from_str("T").unwrap());
    }
}
