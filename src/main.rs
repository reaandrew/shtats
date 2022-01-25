use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Default, Clone, PartialEq)]
struct GitCommit {
    commit_hash: String,
    tags: Vec<String>,
    author: String,
    date: String,
    message: Vec<String>,
    files: Vec<String>,
    lines: Vec<String>,
}

fn chomp_commit(line: &String, commit: &mut GitCommit) {
    let commit_hash = &line[..39];
    let remainder = &line[39..]
        .replace(&['(', ')'][..], "");
    let tags = remainder
        .split(",")
        .filter(|x| x.contains("tag"))
        .map(|x| String::from(&x[5..]))
        .collect::<Vec<String>>();

    commit.commit_hash = commit_hash.into();
    commit.tags = tags;
}

fn process(commit: &GitCommit, stat_functions: &Vec<Box<dyn GitStat>>, stats: &mut GitStats) {
    println!("LINE: {}", &commit.commit_hash);
    for stat in stat_functions{
        stat.process(commit, stats);
    }
}

trait GitStat {
    fn process(&self, commit: &GitCommit, stats: &mut GitStats);
}

struct OverallCommitCount{

}

impl GitStat for OverallCommitCount{
    fn process(&self, _: &GitCommit, stats: &mut GitStats) {
        stats.commit_count += 1
    }
}

#[derive(Default, Clone, PartialEq, Copy)]
struct GitStats{
    commit_count: i32
}

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    let stats_functions: Vec<Box<dyn GitStat>> = vec![
        Box::new(OverallCommitCount{})
    ];

    let mut stats: GitStats = Default::default();

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

    let stdout = Command::new("git")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn git process.")
        .stdout
        .expect("Could not capture standard output.");

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
                process(&current, &stats_functions, &mut stats);
                current = Default::default();
                chomp_commit(&s, &mut current);
            }
            ['A', 'u', 't', 'h', 'o', 'r', ..] => {
                current.author = s.clone();
            }
            ['D', 'a', 't', 'e', ..] => {
                current.date = s.clone();
            }
            [':', ..] => {
                current.files.push(s.clone())
            }
            [a, ..] if a.is_numeric() => {
                current.lines.push(s.clone())
            }
            &[] => {}
            &[_, ..] => {
                current.message.push(s.clone())
            }
        }
    }
    process(&current, &stats_functions, &mut stats);

    println!("Commit Count {}", stats.commit_count);
}

mod tests {}
