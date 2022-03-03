use std::path::Path;
use std::process::exit;
use clap::{Arg, ArgMatches, Command};
use shtats::html::HtmlReporter;
use shtats::output::BufferedOutput;
use shtats::process::{Config, get_number_of_commits, run_shtats};
use git_version::git_version;
use indicatif::ProgressBar;

const GIT_VERSION: &str = git_version!();

fn main() {
    // TODO: Duplicate Commit Messages
    // TODO: Duplicate Commit Messages by user
    // TODO: Commits By Year if no time filter has been applied
    let app_matches = get_app_matches();
    let config = map_config(app_matches);

    let number_of_commits = get_number_of_commits();
    let pb = ProgressBar::new(number_of_commits as u64);
    let mut reporter = HtmlReporter::new(BufferedOutput::new());

    let process_callback = || pb.inc(1);

    match run_shtats(Path::new("."),
                     &mut reporter,
                     config,
                     &process_callback) {
        Ok(_) => {
            pb.finish_with_message("done");
        }
        Err(err) => {
            eprintln!("error: {:?}", err);
            exit(-1);
        }
    }
}

fn map_config(app_matches: ArgMatches) -> Config {
    let mut config: Config = Default::default();
    let run_matches = match app_matches.subcommand() {
        Some(("run", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };

    match run_matches
        .value_of_os("until") {
        None => {}
        Some(until) => {
            config.until = Some(String::from(until.to_str().unwrap()))
        }
    };

    match run_matches
        .value_of_os("since") {
        None => {}
        Some(since) => {
            config.since = Some(String::from(since.to_str().unwrap()))
        }
    };

    match run_matches
        .value_of_os("output") {
        None => {}
        Some(output) => {
            config.output = String::from(output.to_str().unwrap())
        }
    };
    config
}

fn get_app_matches() -> ArgMatches {
    let app_matches = Command::new("Shtats")
        .author("Andy Rea, email@andrewrea.co.uk")
        .version(GIT_VERSION)
        .about("Explains in brief what the program does")
        .subcommand(Command::new("run").arg(
            Arg::new("until")
                .long("until")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .help("gather stats on all commits until this date"),
        ).arg(
            Arg::new("since")
                .long("since")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .help("gather stats on all commits since this date"),
        ).arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .default_value("report.html")
                .help("output filename to write the report to"),
        ), )
        .get_matches();
    app_matches
}
