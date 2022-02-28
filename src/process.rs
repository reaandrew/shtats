use std::io::Error;
use std::path::Path;
use std::process::{ChildStdout, Command, Stdio};
use crate::{BufferedOutput, create_stat_collectors, GitCommit, GitStat, GitStats, GitStatsViewModel, Reporter};
use crate::parsers::parse_git_log;

#[derive(Default)]
pub struct Config {
    pub until: Option<String>,
    pub since: Option<String>,
}

pub fn process_commit(commit: &GitCommit, stat_functions: &Vec<Box<dyn GitStat>>, stats: &mut GitStats) {
    stats.count += 1;
    for stat in stat_functions {
        stat.process(commit, stats);
    }
}

pub fn run_shtats(path: &Path, output: &mut BufferedOutput, reporter: Box<dyn Reporter>, config: Config) -> Result<(), Error> {
    let stats_functions = create_stat_collectors();
    let mut stats: GitStats = GitStats::new(10);
    let args = create_git_log_args(config);
    let stdout = execute_git(args, path);

    parse_git_log(&stats_functions, &mut stats, stdout);

    let viewmodel = GitStatsViewModel::new(&stats.clone());
    reporter.write(output, viewmodel);

    // for item in stats.dup_detector.results(){
    //     println!("DUP {}: {}", item.count, item.duplicate);
    // }

    Ok(())
}


fn execute_git(args: Vec<String>, path: &Path) -> ChildStdout {
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

fn create_git_log_args(config: Config) -> Vec<String> {
    let mut args = vec![
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
    match config.until {
        None => {}
        Some(until) => {
            args.push(String::from("--until"));
            args.push(until);
        }
    }
    match config.since {
        None => {}
        Some(since) => {
            args.push(String::from("--since"));
            args.push(since);
        }
    }
    args
}