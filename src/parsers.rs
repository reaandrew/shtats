use std::io::{BufRead, BufReader};
use std::process::ChildStdout;
use std::str::FromStr;
use chrono::DateTime;
use crate::{GitStat, GitStats};
use crate::models::{FileOperation, GitCommit, LineStat, Operation};
use crate::process::process_commit;

/// Extracts the commit hash and the tags from the line
/// TODO: Split into two separate functions
fn parse_commit(line: &String, commit: &mut GitCommit) {
    let commit_hash = &line[7..47];
    let remainder = &line[47..]
        .replace(&['(', ')'][..], "");
    let tags = remainder
        .split(",")
        .filter(|x| x.contains("tag"))
        .map(|x| String::from(&x[6..]))
        .collect::<Vec<String>>();

    commit.commit_hash = commit_hash.into();
    commit.tags = tags;
}

/// Extracts the raw author string from the line i.e. Name <Email>
fn parse_author(line: &String, commit: &mut GitCommit) {
    commit.author = String::from(&line[8..]);
}

/// Extracts the date from the line
fn parse_date(line: &String, commit: &mut GitCommit) {
    let rfc2822 = DateTime::parse_from_rfc2822(&line[8..].trim()).unwrap();
    commit.date = rfc2822;
}

/// Extracts the file operation for the specified file. e.g. Added, Modified, Deleted, Renamed
fn parse_file_operation(line: &String, commit: &mut GitCommit) {
    let items = line.split("\t").collect::<Vec<&str>>();

    let operation_value = items[0][31..].trim();
    let operation = Operation::from_str(operation_value).expect("failed to parse file operation");

    let file_value = items[1].trim();

    commit.file_operations.push(FileOperation{
        op: operation,
        file: String::from(file_value),
    });
}

/// Extracts the lines added and lines deleted for the specified file from the line
fn parse_line_stats(line: &String, commit: &mut GitCommit) {
    let mut raw = line.split_whitespace();
    let lines_added_raw = String::from(raw.next().expect("failed to split lines added"));
    let lines_deleted_raw = String::from(raw.next().expect("failed to split lines deleted"));

    commit.line_stats.push(LineStat {
        lines_added: lines_added_raw.trim().parse().expect("err parsing lines added"),
        lines_deleted: lines_deleted_raw.trim().parse().expect("error parsing lines deleted"),
    });
}

/// Extracts the commit message from the line and pushes into a list on the GitCommit
fn parse_message(line: &String, commit: &mut GitCommit) {
    commit.message.push(String::from(line));
}

pub(crate) fn parse_git_log(stats_functions: &Vec<Box<dyn GitStat>>, mut stats: &mut GitStats, stdout: ChildStdout) {
    let mut reader = BufReader::new(stdout);
    let mut current: GitCommit = GitCommit::default();
    let mut s = String::new();
    let mut start = true;
    loop {
        s.clear();
        let res = reader.read_line(&mut s);
        if res.is_err() || res.unwrap() == 0 {
            break;
        }
        match s.chars().collect::<Vec<char>>().as_slice() {
            ['c', 'o', 'm', 'm', 'i', 't', ..] => {
                // TODO: Remove the first call when there is no commit to process
                if start {
                    start = false;
                } else {
                    process_commit(&current, &stats_functions, &mut stats);
                }
                current = GitCommit::default();
                parse_commit(&s, &mut current);
            }
            ['A', 'u', 't', 'h', 'o', 'r', ..] => {
                parse_author(&s, &mut current);
            }
            ['D', 'a', 't', 'e', ..] => {
                parse_date(&s, &mut current);
            }
            [':', ..] => {
                parse_file_operation(&s, &mut current);
            }
            [a, ..] if a.is_numeric() => {
                parse_line_stats(&s, &mut current);
            }
            &[] => {}
            &[_, ..] => {
                parse_message(&s, &mut current);
            }
        }
    }
    process_commit(&current, &stats_functions, &mut stats);
}

#[cfg(test)]
mod parse_tests {
    use chrono::{DateTime};
    use crate::{parsers};
    use crate::models::GitCommit;
    use crate::models::Operation::{ADD, DELETE, MODIFY, RENAME};

    #[test]
    fn test_parse_commit() {
        let mut commit: GitCommit = GitCommit::default();
        let line = "commit aa7072f9497aa607f2381ec7a1d5f1f638ed57ae (HEAD -> main, origin/main)";
        parsers::parse_commit(&String::from(line), &mut commit);
        assert_eq!("aa7072f9497aa607f2381ec7a1d5f1f638ed57ae", commit.commit_hash);
    }

    #[test]
    fn test_parse_commit_parses_tags() {
        let mut commit: GitCommit = GitCommit::default();
        let line = "commit 87f0e98024ff172c8836ec082e6993ad46865e64 (tag: v0.1.0)";
        parsers::parse_commit(&String::from(line), &mut commit);
        assert_eq!(1, commit.tags.len());
        assert_eq!("v0.1.0", commit.tags.get(0).unwrap());
    }

    #[test]
    fn test_parse_author() {
        let mut commit: GitCommit = GitCommit::default();
        let line = "Author: Andy Rea <test@does-not-exist.com>";
        parsers::parse_author(&String::from(line), &mut commit);
        assert_eq!("Andy Rea <test@does-not-exist.com>", commit.author);
    }

    #[test]
    fn test_parse_date() {
        let mut commit: GitCommit = GitCommit::default();
        let line = "Date:   Wed, 2 Feb 2022 12:02:17 +0000";
        parsers::parse_date(&String::from(line), &mut commit);
        assert_eq!(DateTime::parse_from_rfc2822("Wed, 2 Feb 2022 12:02:17 +0000").unwrap(), commit.date);
    }

    #[test]
    fn test_parse_filestats() {
        let mut commit: GitCommit = GitCommit::default();
        let lines = vec![
            ":000000 100644 0000000 5ebc4f7 A\t.github/workflows/rust.yml",
            ":000000 100644 0000000 ea8c4bf M\t.gitignore",
            ":000000 100644 0000000 696dd88 D\tCargo.lock",
            ":000000 100644 0000000 94adb32 R100\tCargo.toml",
            ":000000 100644 0000000 3fb8f3d A\tLICENSE.md",
            ":000000 100644 0000000 8460aee A\tREADME.md",
            ":000000 100644 0000000 3a5a08a A\tsrc/main.rs",
            ":000000 100644 0000000 1e94b1b A\tDapper.EntityFramework NET40/App.config"
        ];

        for line in lines {
            parsers::parse_file_operation(&String::from(line), &mut commit);
        }

        assert_eq!(8, commit.file_operations.len());
        assert_eq!(ADD, commit.file_operations[0].op);
        assert_eq!(".github/workflows/rust.yml", commit.file_operations[0].file);
        assert_eq!(MODIFY, commit.file_operations[1].op);
        assert_eq!(".gitignore", commit.file_operations[1].file);
        assert_eq!(DELETE, commit.file_operations[2].op);
        assert_eq!("Cargo.lock", commit.file_operations[2].file);
        assert_eq!(RENAME, commit.file_operations[3].op);
        assert_eq!("Cargo.toml", commit.file_operations[3].file);

        assert_eq!("Dapper.EntityFramework NET40/App.config", commit.file_operations[7].file);
    }

    #[test]
    fn test_parse_line_stats() {
        let mut commit: GitCommit = GitCommit::default();
        let lines = vec![
            "142     0       .github/workflows/rust.yml",
            "1       2123    .gitignore",
            "7       0       Cargo.lock",
        ];

        for line in lines {
            parsers::parse_line_stats(&String::from(line), &mut commit);
        }

        assert_eq!(3, commit.line_stats.len());
        assert_eq!(142, commit.line_stats.get(0).unwrap().lines_added);
        assert_eq!(0, commit.line_stats.get(0).unwrap().lines_deleted);
        assert_eq!(1, commit.line_stats.get(1).unwrap().lines_added);
        assert_eq!(2123, commit.line_stats.get(1).unwrap().lines_deleted);
        assert_eq!(7, commit.line_stats.get(2).unwrap().lines_added);
        assert_eq!(0, commit.line_stats.get(2).unwrap().lines_deleted);
    }

    #[test]
    fn test_parse_message() {
        let mut commit: GitCommit = GitCommit::default();
        let lines = vec![
            "    This is a massive commit:",
            "    ",
            "    Author in message: Something",
            "    Subject in message: Somthing else",
            "    commit in message",
            "    Date in message: BOOM",
        ];

        for line in lines {
            parsers::parse_message(&String::from(line), &mut commit);
        }

        assert_eq!(6, commit.message.len());
    }
}

