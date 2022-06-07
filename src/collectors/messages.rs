use bytesize::ByteSize;
use serde_json::Error;
use crate::{GitCommit, GitStat};
use crate::stats::{JsonValue, MessageStats};
use crate::viewmodel::{GitStatsJsonViewModelItem, SummaryViewModelItem};

pub struct MessagesCollector {
    count: i32,
    total_message_lines: i32,
    total_message_size: i32,
    message_stats: MessageStats,
}


impl MessagesCollector {
    pub fn default() -> Self {
        Self {
            count: 0,
            total_message_lines: 0,
            total_message_size: 0,
            message_stats: Default::default(),
        }
    }
}

impl JsonValue for MessagesCollector {
    fn get_json_viewmodel(&self) -> Result<GitStatsJsonViewModelItem, Error> {
        let mut summary: Vec<SummaryViewModelItem> = Default::default();
        summary.push(SummaryViewModelItem {
            name: "Total size of all commit messages".to_string(),
            value: ByteSize(self.total_message_size as u64).to_string(),
        });

        summary.push(SummaryViewModelItem {
            name: "Total number of lines_collection across all commit messages".to_string(),
            value: self.total_message_lines.to_string(),
        });

        summary.push(SummaryViewModelItem {
            name: "Max number of lines_collection in a commit message".to_string(),
            value: self.message_stats.max_lines.to_string(),
        });

        summary.push(SummaryViewModelItem {
            name: "Max size of a commit message".to_string(),
            value: ByteSize(self.message_stats.max_size as u64).to_string(),
        });

        summary.push(SummaryViewModelItem {
            name: "Avg number of lines_collection in a commit message".to_string(),
            value: self.message_stats.avg_lines.to_string(),
        });

        summary.push(SummaryViewModelItem {
            name: "Avg size of a commit message".to_string(),
            value: ByteSize(self.message_stats.avg_size as u64).to_string(),
        });

        return Ok(GitStatsJsonViewModelItem {
            summary,
            key: String::from("commit_message_stats"),
            data: Default::default(),
        });
    }
}

impl GitStat for MessagesCollector {
    fn process(&mut self, commit: &GitCommit) {
        self.count += 1;
        self.total_message_lines += commit.total_message_lines();
        self.total_message_size += commit.total_message_size();

        if commit.total_message_size() > self.message_stats.max_size {
            self.message_stats.max_size = commit.total_message_size()
        }

        if commit.total_message_lines() > self.message_stats.max_lines {
            self.message_stats.max_lines = commit.total_message_lines();
        }

        if commit.total_message_size() <= self.message_stats.min_size {
            self.message_stats.min_size = commit.total_message_size()
        }

        if commit.total_message_lines() <= self.message_stats.min_lines {
            self.message_stats.min_lines = commit.total_message_lines();
        }

        self.message_stats.avg_size = self.total_message_size / self.count;
        self.message_stats.avg_lines = self.total_message_lines / self.count;
    }
}

#[cfg(test)]
mod tests{
    use crate::{GitCommit, GitStat};
    use crate::collectors::messages::MessagesCollector;
    use crate::stats::JsonValue;

    #[test]
    fn test_process(){
        let mut subject = MessagesCollector::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.message.push(String::from("line 1"));
        commit.message.push(String::from("line 2"));
        commit.message.push(String::from("line 3"));
        commit.message.push(String::from("line 4"));
        subject.process(&commit);

        assert_eq!(subject.count, 1);
        assert_eq!(subject.total_message_lines, 4);
        assert_eq!(subject.total_message_size, 24);
    }

    #[test]
    fn test_json_viewmodel(){
        let mut subject = MessagesCollector::default();
        let mut commit: GitCommit = GitCommit::default();
        commit.message.push(String::from("line 1"));
        commit.message.push(String::from("line 2"));
        commit.message.push(String::from("line 3"));
        commit.message.push(String::from("line 4"));
        subject.process(&commit);

        let result = subject.get_json_viewmodel().unwrap();
        assert_eq!(result.summary.len(), 6);
    }
}