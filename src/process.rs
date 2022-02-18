use std::io::Error;
use std::path::Path;
use std::process::{ChildStdout, Command, Stdio};
use crate::{BufferedOutput, create_stat_collectors, GitCommit, GitStat, GitStats, GitStatsViewModel, Reporter};
use crate::parsers::parse_git_log;

pub fn process_commit(commit: &GitCommit, stat_functions: &Vec<Box<dyn GitStat>>, stats: &mut GitStats) {
    stats.count += 1;
    if stats.count %10 == 0{
        println!("{}", stats.count);
    }
    for stat in stat_functions {
        stat.process(commit, stats);
    }
}

pub fn run_shtats(path: &Path, output: &mut BufferedOutput, reporter: Box<dyn Reporter>) -> Result<(), Error> {
    let stats_functions = create_stat_collectors();
    let mut stats: GitStats = GitStats::new(10);
    let args = create_git_log_args();
    let stdout = execute_git(&args, path);

    parse_git_log(&stats_functions, &mut stats, stdout);

    let viewmodel = GitStatsViewModel::new(&stats.clone());
    reporter.write(output, viewmodel);

    for item in stats.dup_detector.results(){
        println!("DUP {}: {}", item.count, item.duplicate);
    }

    Ok(())
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
        "--date-order",
        "--reverse",
        "--numstat",
        "--date=rfc2822",
    ];
    args
}