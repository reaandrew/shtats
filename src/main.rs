use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Default, Clone, PartialEq)]
struct GitCommit {
    commit_hash: String,
    tags: Vec<String>
}

fn chomp_commit(line: &String, commit: &GitCommit) -> GitCommit {
    let commit_hash = &line[..39];
    let remainder = &line[39..]
        .replace(&['(', ')'][..], "");
    let tags = remainder
        .split(",")
        .filter(|x| x.contains("tag"))
        .map(|x| String::from(&x[5..]))
        .collect::<Vec<String>>();

    let mut commit_clone = commit.clone();
    commit_clone.commit_hash = commit_hash.into();
    commit_clone.tags = tags;

    return commit_clone;
}

fn process(_: &GitCommit, stats: &mut Stats){
    stats.commit_count += 1;

    if stats.commit_count %1000 == 0 {
        println!("processed {}", stats.commit_count);
    }
}

#[derive(Default)]
struct Stats{
    commit_count: i32
}

fn main() {
    let args = vec![
        "--no-pager",
        "log",
        "-z",
        "--pretty=format:%ncommit %w(0,0,1) %H%d%nAuthor: %an <%ae> %nDate: %ad%nSubject: %s%b",
        "--tags",
        "--all",
        "--raw",
        "--date-order",
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

    loop{
        s.clear();
        let res = reader.read_line(&mut s);
        if res.is_err() || res.unwrap() == 0 {
            break;
        }
        println!("{}", s);
        match s.chars().collect::<Vec<char>>().as_slice() {
            ['c', 'o', 'm', 'm', 'i', 't', ..] => {
                if current != Default::default(){
                    process(&current, &mut stats);
                }
                current = Default::default();
                current = chomp_commit(&s, &current);
            }
            &[] => {}
            &[_, ..] => {}
        }
    }

    //  U+0000
    //
    // reader
    //     .lines()
    //     .filter_map(|x|x.ok())
    //     .for_each(|line| {
    //         println!("{}", line);
    //         match line.chars().collect::<Vec<char>>().as_slice() {
    //             ['c', 'o', 'm', 'm', 'i', 't', ..] if current.commit_hash != "" => {
    //                 if current != Default::default(){
    //                     process(&current, &mut stats);
    //                 }
    //                 current = Default::default();
    //                 current = chomp_commit(line, &current);
    //             }
    //             &[] => {}
    //             &[_, ..] => {}
    //         }
    //     });
    //
    // process(&current, &mut stats);

    println!("Commit Count: {}", stats.commit_count);
}

mod tests {}
