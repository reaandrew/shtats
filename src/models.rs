use std::str::FromStr;
use chrono::{DateTime, FixedOffset, Utc};
use crate::models::Operation::{ADD, DELETE, MODIFY, RENAME};

#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub(crate) enum Operation {
    ADD = b'A',
    MODIFY = b'M',
    DELETE = b'D',
    RENAME = b'R',
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Operation, ()> {
        match s.chars().collect::<Vec<char>>().as_slice() {
            ['A'] => Ok(Operation::ADD),
            ['M'] => Ok(Operation::MODIFY),
            ['D'] => Ok(Operation::DELETE),
            ['R', ..] => Ok(Operation::RENAME),
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
    pub(crate) author: String,
    pub(crate) date: DateTime<FixedOffset>,
    pub(crate) message: Vec<String>,
    pub(crate) file_operations: Vec<FileOperation>,
    pub(crate) line_stats: Vec<LineStat>,
}

impl GitCommit {
    pub(crate) fn default() -> Self {
        return GitCommit {
            commit_hash: "".to_string(),
            tags: vec![],
            author: "".to_string(),
            date: DateTime::from(Utc::now()),
            message: vec![],
            file_operations: vec![],
            line_stats: vec![],
        };
    }

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

    pub(crate) fn total_files_added(&self) -> i32 {
        return self.file_operations.iter().filter(|x| x.op == ADD).count() as i32;
    }

    pub(crate) fn total_files_deleted(&self) -> i32 {
        return self.file_operations.iter().filter(|x| x.op == DELETE).count() as i32;
    }

    pub(crate) fn total_files_modified(&self) -> i32 {
        return self.file_operations.iter().filter(|x| x.op == MODIFY).count() as i32;
    }

    pub(crate) fn total_files_renamed(&self) -> i32 {
        return self.file_operations.iter().filter(|x| x.op == RENAME).count() as i32;
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
