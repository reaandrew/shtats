use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Default, Clone, PartialEq)]
struct GitCommit {
    commit_hash: String,
    tags: Vec<String>,
    Author: String,
    Date: String,
    Message: Vec<String>,
    Files: Vec<String>,
    Lines: Vec<String>,
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

fn process(commit: &GitCommit, stats: &mut Stats) {
    stats.commit_count += 1;

    if stats.commit_count % 1000 == 0 {
        println!("processed {}", stats.commit_count);
    }
}

#[derive(Default)]
struct Stats {
    commit_count: i32,
}

fn main() {
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

    let mut stats: Stats = Default::default();

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
                process(&current, &mut stats);
                current = Default::default();
                chomp_commit(&s, &mut current);
            }
            ['A', 'u', 't', 'h', 'o', 'r', ..] => {
                current.Author = s.clone();
            }
            ['D', 'a', 't', 'e', ..] => {
                current.Date = s.clone();
            }
            [':', ..] => {
                current.Files.push(s.clone())
            }
            [a, ..] if a.is_numeric() => {
                current.Lines.push(s.clone())
            }
            &[] => {}
            &[_, ..] => {
                current.Message.push(s.clone())
            }
        }
    }
    process(&current, &mut stats);

    println!("Commit Count: {}", stats.commit_count);
}

mod tests {}
