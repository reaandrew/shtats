use std::io::{BufRead, BufReader, Error};
use std::path::{Path};
use std::process::{ChildStdout, Command, Stdio};

pub struct BufferedOutput {
    data: String,
}

impl BufferedOutput {
    pub fn new() -> Self {
        Self { data: "".into() }
    }

    pub fn write(&mut self, data: String) {
        self.data = data;
    }

    pub fn to_string(&self) -> String{
        return self.data.clone();
    }
}

#[derive(Default, Clone, PartialEq)]
struct GitCommit {
    commit_hash: String,
    tags: Vec<String>,
    author: String,
    date: String,
    message: Vec<String>,
    file_operations: Vec<String>,
    line_stats: Vec<String>,
}

/// Extracts the commit hash and the tags from the line
/// TODO: Split into two separate functions
fn chomp_commit(line: &String, commit: &mut GitCommit) {
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
fn chomp_author(line: &String, commit: &mut GitCommit) {
    commit.author = String::from(&line[8..]);
}

/// Extracts the date from the line
fn chomp_date(line: &String, commit: &mut GitCommit) {
    commit.date = String::from(&line[8..]);
}

/// Extracts the file operation for the specified file. e.g. Added, Modified, Deleted, Renamed
fn chomp_file_operation(line: &String, commit: &mut GitCommit) {
    commit.file_operations.push(String::from(line));
}

/// Extracts the lines added and lines deleted for the specified file from the line
fn chomp_line_stats(line: &String, commit: &mut GitCommit) {
    commit.line_stats.push(String::from(line));
}

/// Extracts the commit message from the line and pushes into a list on the GitCommit
fn chomp_message(line: &String, commit: &mut GitCommit) {
    commit.message.push(String::from(line));
}

fn process_commit(commit: &GitCommit, stat_functions: &Vec<Box<dyn GitStat>>, stats: &mut GitStats) {
    for stat in stat_functions {
        stat.process(commit, stats);
    }
}

trait GitStat {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats);
}

struct OverallCommitCount {}

impl GitStat for OverallCommitCount {
    fn process(&self, _: &GitCommit, stats: &mut GitStats) {
        stats.commit_count += 1
    }
}

#[derive(Default, Clone, PartialEq, Copy)]
struct GitStats {
    commit_count: i32,
}

trait Reporter{
    fn write(&self, output: &mut BufferedOutput,  stats: GitStats) ;
}

struct HtmlReporter{

}

impl Reporter for HtmlReporter{
    fn write(&self, output: &mut BufferedOutput,  stats: GitStats) {
        output.write(format!("<div>Count Count: {}</div>", stats.commit_count))
    }
}

impl HtmlReporter {
    pub fn new() -> Self {
        HtmlReporter {}
    }
}


pub fn run_forora(path: &Path, output: &mut BufferedOutput) -> Result<(), Error> {
    println!("{}", path.to_str().unwrap());

    let stats_functions = create_stat_functions();
    let mut stats: GitStats = Default::default();
    let args = create_git_log_args();
    let stdout = execute_git(&args, path);

    process_git_log(&stats_functions, &mut stats, stdout);
    output.write(format!("Count Count: {}", stats.commit_count));
    Ok(())
}


fn process_git_log(stats_functions: &Vec<Box<dyn GitStat>>, mut stats: &mut GitStats, stdout: ChildStdout) {
    let mut reader = BufReader::new(stdout);
    let mut current: GitCommit = Default::default();
    let mut s = String::new();

    loop {
        s.clear();
        let res = reader.read_line(&mut s);
        if res.is_err() || res.unwrap() == 0 {
            break;
        }
        match s.chars().collect::<Vec<char>>().as_slice() {
            ['c', 'o', 'm', 'm', 'i', 't', ..] => {
                // TODO: Remove the first call when there is no commit to process
                process_commit(&current, &stats_functions, &mut stats);
                current = Default::default();
                chomp_commit(&s, &mut current);
            }
            ['A', 'u', 't', 'h', 'o', 'r', ..] => {
                chomp_author(&s, &mut current);
            }
            ['D', 'a', 't', 'e', ..] => {
                chomp_date(&s, &mut current);
            }
            [':', ..] => {
                chomp_file_operation(&s, &mut current);
            }
            [a, ..] if a.is_numeric() => {
                chomp_line_stats(&s, &mut current);
            }
            &[] => {}
            &[_, ..] => {
                chomp_message(&s, &mut current);
            }
        }
    }
    process_commit(&current, &stats_functions, &mut stats);
}

fn execute_git(args: &Vec<&str>, path: &Path) -> ChildStdout {
    let stdout = Command::new("git")
        .current_dir(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn git process.")
        .stdout
        .expect("Could not capture standard output.");
    stdout
}

fn create_git_log_args() -> Vec<&'static str> {
    let args = vec![
        "--no-pager",
        "log",
        "--all",
        "--raw",
        "--decorate",
        "--reverse",
        "--numstat",
        "--date=rfc2822",
    ];
    args
}

fn create_stat_functions() -> Vec<Box<dyn GitStat>> {
    let stats_functions: Vec<Box<dyn GitStat>> = vec![
        Box::new(OverallCommitCount {})
    ];
    stats_functions
}

#[cfg(test)]
mod chomp_tests {
    use crate::{chomp_author, chomp_commit, chomp_date, chomp_file_operation, chomp_line_stats, chomp_message, GitCommit};

    #[test]
    fn test_chomp_commit() {
        let mut commit: GitCommit = Default::default();
        let line = "commit aa7072f9497aa607f2381ec7a1d5f1f638ed57ae (HEAD -> main, origin/main)";
        chomp_commit(&String::from(line), &mut commit);
        assert_eq!("aa7072f9497aa607f2381ec7a1d5f1f638ed57ae", commit.commit_hash);
    }

    #[test]
    fn test_chomp_commit_parses_tags() {
        let mut commit: GitCommit = Default::default();
        let line = "commit 87f0e98024ff172c8836ec082e6993ad46865e64 (tag: v0.1.0)";
        chomp_commit(&String::from(line), &mut commit);
        assert_eq!(1, commit.tags.len());
        assert_eq!("v0.1.0", commit.tags.get(0).unwrap());
    }

    #[test]
    fn test_chomp_author() {
        let mut commit: GitCommit = Default::default();
        let line = "Author: Andy Rea <test@does-not-exist.com>";
        chomp_author(&String::from(line), &mut commit);
        assert_eq!("Andy Rea <test@does-not-exist.com>", commit.author);
    }

    #[test]
    fn test_chomp_date() {
        let mut commit: GitCommit = Default::default();
        let line = "Date:   Fri Jan 21 13:29:22 2022 +0000";
        chomp_date(&String::from(line), &mut commit);
        assert_eq!("Fri Jan 21 13:29:22 2022 +0000", commit.date);
    }

    #[test]
    fn test_chomp_filestats() {
        let mut commit: GitCommit = Default::default();
        let lines = vec![
            ":000000 100644 0000000 5ebc4f7 A        .github/workflows/rust.yml",
            ":000000 100644 0000000 ea8c4bf A        .gitignore",
            ":000000 100644 0000000 696dd88 A        Cargo.lock",
            ":000000 100644 0000000 94adb32 A        Cargo.toml",
            ":000000 100644 0000000 3fb8f3d A        LICENSE.md",
            ":000000 100644 0000000 8460aee A        README.md",
            ":000000 100644 0000000 3a5a08a A        src/main.rs",
        ];

        for line in lines {
            chomp_file_operation(&String::from(line), &mut commit);
        }

        assert_eq!(7, commit.file_operations.len());
    }

    #[test]
    fn test_chomp_line_stats() {
        let mut commit: GitCommit = Default::default();
        let lines = vec![
            "142     0       .github/workflows/rust.yml",
            "1       0       .gitignore",
            "7       0       Cargo.lock",
        ];

        for line in lines {
            chomp_line_stats(&String::from(line), &mut commit);
        }

        assert_eq!(3, commit.line_stats.len());
    }

    #[test]
    fn test_chomp_message() {
        let mut commit: GitCommit = Default::default();
        let lines = vec![
            "    This is a massive commit:",
            "    ",
            "    Author in message: Something",
            "    Subject in message: Somthing else",
            "    commit in message",
            "    Date in message: BOOM",
        ];

        for line in lines {
            chomp_message(&String::from(line), &mut commit);
        }

        assert_eq!(6, commit.message.len());
    }
}

#[cfg(test)]
mod stat_tests {
    use crate::{GitCommit, GitStat, GitStats, OverallCommitCount, process_commit};

    #[test]
    fn test_overall_commit_count_with_1_commit() {
        let mut commit: GitCommit = Default::default();
        commit.commit_hash = String::from("123");

        let stat_functions: Vec<Box<dyn GitStat>> = vec![
            Box::new(OverallCommitCount {})
        ];

        let mut stats: GitStats = Default::default();

        process_commit(&commit, &stat_functions, &mut stats);

        assert_eq!(1, stats.commit_count);
    }
}