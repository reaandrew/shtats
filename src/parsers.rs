use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::ChildStdout;
use std::str::FromStr;
use chrono::{DateTime, FixedOffset};
use crate::errors::ShtatsError;
use crate::models::{FileOperation, GitAuthor, GitCommit, LineStat, Operation};
use crate::result::Result;


pub trait GitLogReader {
    fn read_line(&mut self, buf: &mut String) -> Result<usize>;
}

pub struct GitCommitIterator {
    finished: bool,
    buffer: String,
    reader: Box<dyn GitLogReader>,
    current: GitCommit,
    next: GitCommit,
    parser: GitCommitParser,
}

impl GitCommitIterator {
    pub(crate) fn new(reader: Box<dyn GitLogReader>) -> GitCommitIterator {
        return GitCommitIterator {
            finished: false,
            buffer: String::new(),
            reader,
            current: GitCommit::default(),
            next: GitCommit::default(),
            parser: GitCommitParser::new(),
        };
    }
}

impl Iterator for GitCommitIterator {
    type Item = GitCommit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.next != GitCommit::default() {
            self.current = self.next.clone();
        }
        loop {
            self.buffer.clear();
            let res = self.reader.read_line(&mut self.buffer);
            if res.is_err() || res.unwrap() == 0 {
                self.finished = true;
                break;
            }
            let line = self.buffer.clone();

            match self.buffer.chars().collect::<Vec<char>>().as_slice() {
                ['c', 'o', 'm', 'm', 'i', 't', ..] => {
                    if self.current == GitCommit::default() {
                        self.current.commit_hash = self.parser.parse_commit_hash(&line);
                        self.current.tags = self.parser.parse_commit_tags(&line);
                    } else {
                        self.next = GitCommit::default();
                        self.next.commit_hash = self.parser.parse_commit_hash(&line);
                        self.next.tags = self.parser.parse_commit_tags(&line);
                        break;
                    }
                }
                ['A', 'u', 't', 'h', 'o', 'r', ..] => {
                    self.current.author = self.parser.parse_author(&line);
                }
                ['D', 'a', 't', 'e', ..] => {
                    self.current.date = self.parser.parse_date(&line);
                }
                [':', ..] => {
                    self.current.file_operations.push(self.parser.parse_file_operation(&line));
                }
                [a, ..] if a.is_numeric() => {
                    self.current.line_stats.push(self.parser.parse_line_stats(&line));
                }
                &[] => {}
                &[_, ..] => {
                    self.current.message.push(self.parser.parse_message(&line));
                }
            }
        };
        let curr = self.current.clone();
        Some(curr)
    }
}

pub struct StdoutGitLogReader {
    pub(crate) stdout: BufReader<ChildStdout>,
}

impl GitLogReader for StdoutGitLogReader {
    fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        match self.stdout.read_line(buf) {
            Ok(val) => { Ok(val) }
            Err(err) => { Err(ShtatsError::from(err)) }
        }
    }
}

struct GitCommitParser {}

impl GitCommitParser {
    fn new() -> GitCommitParser {
        return GitCommitParser {};
    }

    /// Extracts the commit hash from the line
    fn parse_commit_hash(&mut self, line: &String) -> String {
        let commit_hash = &line[7..47];
        return commit_hash.into();
    }

    /// Extracts the tags from the line
    fn parse_commit_tags(&mut self, line: &String) -> Vec<String> {
        let remainder = &line[47..]
            .replace(&['(', ')'][..], "");
        return remainder
            .split(",")
            .filter(|x| x.contains("tag"))
            .map(|x| String::from(&x[6..]))
            .collect::<Vec<String>>();
    }

    /// Extracts the raw author string from the line i.e. Name <Email>
    fn parse_author(&self, line: &String) -> GitAuthor {
        let author = String::from(line[8..].trim());
        let items = author.split('<').collect::<Vec<&str>>();
        let name = items[0].trim();
        let email = items[1].replace(">", "");
        return GitAuthor {
            name: String::from(name),
            email: String::from(email),
        };
    }

    /// Extracts the date from the line
    fn parse_date(&self, line: &String) -> DateTime<FixedOffset> {
        let rfc2822 = DateTime::parse_from_rfc2822(&line[8..].trim()).unwrap();
        return rfc2822;
    }

    /// Extracts the file operation for the specified file. e.g. Added, Modified, Deleted, Renamed
    fn parse_file_operation(&self, line: &String) -> FileOperation {
        let items = line.split("\t").collect::<Vec<&str>>();

        let operation_value = items[0].split_whitespace().last().expect("Could not extract operation").trim();

        let operation = Operation::from_str(operation_value).expect("Could not parse operation");

        let file_value = items[1].trim();

        let filename = file_value;

        let extension = String::from(Path::new(filename)
            .extension()
            .and_then(OsStr::to_str).unwrap_or(""));

        return FileOperation {
            op: operation,
            file: String::from(filename),
            file_extension: extension,
        };
    }

    /// Extracts the lines added and lines deleted for the specified file from the line
    fn parse_line_stats(&self, line: &String) -> LineStat {
        let mut raw = line.split_whitespace();
        let lines_added_raw = String::from(raw.next().expect("failed to split lines added"));
        let lines_deleted_raw = String::from(raw.next().expect("failed to split lines deleted"));
        let file = String::from(raw.next().expect("failed to split file name for line stats"));

        return LineStat {
            lines_added: lines_added_raw.trim().parse().expect("err parsing lines added"),
            lines_deleted: lines_deleted_raw.trim().parse().expect("error parsing lines deleted"),
            file,
        };
    }

    /// Extracts the commit message from the line and pushes into a list on the GitCommit
    fn parse_message(&self, line: &String) -> String {
        return String::from(line);
    }
}

#[cfg(test)]
mod commit_iterator_tests {
    use std::fmt::Write;
    use crate::parsers::{GitCommitIterator, GitLogReader};
    use crate::{result};

    struct FakeGitLogReader<'a> {
        items: Vec<&'a str>,
        index: usize,
    }

    impl FakeGitLogReader<'_> {
        fn new(items: Vec<&str>) -> FakeGitLogReader {
            return FakeGitLogReader {
                items,
                index: 0,
            };
        }
    }

    impl GitLogReader for FakeGitLogReader<'_> {
        fn read_line(&mut self, buf: &mut String) -> result::Result<usize> {
            if self.index == self.items.len() {
                Ok(0)
            } else {
                let item = self.items.get(self.index).unwrap();
                let _ = buf.write_str(item);
                self.index += 1;
                Ok(item.len())
            }
        }
    }


    #[test]
    fn test_fake_git_log_reader() {
        let mut s = String::new();
        let mut reader = FakeGitLogReader::new(vec![
            "something"
        ]);
        let result_1 = reader.read_line(&mut s).unwrap();
        let result_2 = reader.read_line(&mut s).unwrap();
        assert_eq!(result_1, "something".len());
        assert_eq!(result_2, 0);
    }

    #[test]
    fn test_iterator_separates_commits() {
        let reader = FakeGitLogReader::new(vec![
            "commit aa7072f9497aa607f2381ec7a1d5f1f638ed57a1 (HEAD -> main, origin/main)",
            "commit aa7072f9497aa607f2381ec7a1d5f1f638ed57a2",
            "commit aa7072f9497aa607f2381ec7a1d5f1f638ed57a3",
        ]);
        let iterator = GitCommitIterator::new(Box::new(reader));

        println!("{}", iterator.count());
    }
}

#[cfg(test)]
mod stdout_commit_iterator_integration_tests {
    use std::io::{BufReader};
    use std::path::Path;
    use std::process::{Command, Stdio};
    use crate::parsers::{GitCommitIterator, StdoutGitLogReader};

    #[test]
    fn test_iterator_separates_commits() {
        let args = vec![
            String::from("--no-pager"),
            String::from("log"),
            String::from("--all"),
            String::from("--raw"),
            String::from("--decorate"),
            String::from("--date-order"),
            String::from("--reverse"),
            String::from("--numstat"),
            String::from("--date=rfc2822"),
        ];

        let stdout = Command::new("git")
            .current_dir(Path::new("."))
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Could not spawn git process.")
            .stdout
            .expect("Could not capture standard output.");

        let buf_reader = BufReader::new(stdout);
        let reader = StdoutGitLogReader { stdout: buf_reader };
        let iterator = GitCommitIterator::new(Box::new(reader));

        let mut count = 1;
        for _ in iterator.into_iter() {
            count += 1;
        }

        println!("Count {}", count);
        //Pretty useless assertion but a simple assertion to tie off actually running the reader.
        assert!(count > 1)
    }
}

#[cfg(test)]
mod commit_parser_tests {
    use chrono::{DateTime};
    use crate::models::Operation::{Added, Deleted, Modified, Renamed};
    use crate::parsers::GitCommitParser;

    #[test]
    fn test_parse_commit() {
        let line = "commit aa7072f9497aa607f2381ec7a1d5f1f638ed57ae (HEAD -> main, origin/main)";
        let actual = GitCommitParser::new().parse_commit_hash(&String::from(line));
        assert_eq!("aa7072f9497aa607f2381ec7a1d5f1f638ed57ae", actual);
    }

    #[test]
    fn test_parse_commit_parses_tags() {
        let line = "commit 87f0e98024ff172c8836ec082e6993ad46865e64 (tag: v0.1.0)";
        let actual = GitCommitParser::new().parse_commit_tags(&String::from(line));
        assert_eq!(1, actual.len());
        assert_eq!("v0.1.0", actual.get(0).unwrap());
    }

    #[test]
    fn test_parse_author() {
        let line = "Author: Andy Rea <test@does-not-exist.com>";
        let actual = GitCommitParser::new().parse_author(&String::from(line));
        assert_eq!("Andy Rea", actual.name);
        assert_eq!("test@does-not-exist.com", actual.email);
    }

    #[test]
    fn test_parse_date() {
        let line = "Date:   Wed, 2 Feb 2022 12:02:17 +0000";
        let actual = GitCommitParser::new().parse_date(&String::from(line));
        assert_eq!(DateTime::parse_from_rfc2822("Wed, 2 Feb 2022 12:02:17 +0000").unwrap(), actual);
    }

    #[test]
    fn test_parse_filestats() {
        let lines = vec![
            ":000000 100644 0000000 5ebc4f7 A\t.github/workflows/rust.yml",
            ":000000 100644 0000000 ea8c4bf M\t.gitignore",
            ":000000 100644 0000000 696dd88 D\tCargo.lock",
            ":000000 100644 0000000 94adb32 R100\tCargo.toml",
            ":000000 100644 0000000 3fb8f3d A\tLICENSE.md",
            ":000000 100644 0000000 8460aee A\tREADME.md",
            ":000000 100644 0000000 3a5a08a A\tsrc/main.rs",
            ":000000 100644 0000000 1e94b1b A\tDapper.EntityFramework NET40/App.config",
            ":100644 100644 7075b69663 2caeec3851 M\tsrc/Microsoft.AspNet.PipelineCore/DefaultHttpResponse.cs",
        ];

        let mut file_ops = vec![];

        for line in lines {
            file_ops.push(GitCommitParser::new().parse_file_operation(&String::from(line)));
        }

        assert_eq!(9, file_ops.len());
        assert_eq!(Added, file_ops[0].op);
        assert_eq!(".github/workflows/rust.yml", file_ops[0].file);
        assert_eq!(Modified, file_ops[1].op);
        assert_eq!(".gitignore", file_ops[1].file);
        assert_eq!(Deleted, file_ops[2].op);
        assert_eq!("Cargo.lock", file_ops[2].file);
        assert_eq!(Renamed, file_ops[3].op);
        assert_eq!("Cargo.toml", file_ops[3].file);
        assert_eq!("Dapper.EntityFramework NET40/App.config", file_ops[7].file);
        assert_eq!(Modified, file_ops[8].op);
        assert_eq!("src/Microsoft.AspNet.PipelineCore/DefaultHttpResponse.cs", file_ops[8].file);
    }

    #[test]
    fn test_parse_line_stats() {
        let lines = vec![
            "142     0       .github/workflows/rust.yml",
            "1       2123    .gitignore",
            "7       0       Cargo.lock",
        ];

        let mut line_stats = vec![];

        for line in lines {
            line_stats.push(GitCommitParser::new().parse_line_stats(&String::from(line)));
        }

        assert_eq!(3, line_stats.len());
        assert_eq!(142, line_stats.get(0).unwrap().lines_added);
        assert_eq!(0, line_stats.get(0).unwrap().lines_deleted);
        assert_eq!(1, line_stats.get(1).unwrap().lines_added);
        assert_eq!(2123, line_stats.get(1).unwrap().lines_deleted);
        assert_eq!(7, line_stats.get(2).unwrap().lines_added);
        assert_eq!(0, line_stats.get(2).unwrap().lines_deleted);
    }

    #[test]
    fn test_parse_message() {
        let lines = vec![
            "    This is a massive commit:",
            "    ",
            "    Author in message: Something",
            "    Subject in message: Somthing else",
            "    commit in message",
            "    Date in message: BOOM",
        ];

        let mut commit_msg = vec![];

        for line in lines {
            commit_msg.push(GitCommitParser::new().parse_message(&String::from(line)));
        }

        assert_eq!(6, commit_msg.len());
    }
}

