use std::path::Path;
use clap::{Arg, Command};
use shtats::html::HtmlReporter;
use shtats::output::BufferedOutput;
use shtats::process::{Config, run_shtats};
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

fn main() {
    //git rev-list --all --count
    //
    //  The above will give you the number of commits first so a progress bar can be displayed.

    // TODO: Duplicate Commit Messages
    // TODO: Duplicate Commit Messages by user
    // TODO: Commits By Year if no time filter has been applied
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
        ), )
        .get_matches();

    let run_matches = match app_matches.subcommand() {
        Some(("run", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };

    let mut config: Config = Default::default();

    match run_matches
        .value_of_os("until"){
        None => {}
        Some(until) => {
            config.until = Some(String::from(until.to_str().unwrap()))
        }
    };

    match run_matches
        .value_of_os("since"){
        None => {}
        Some(since) => {
            config.since = Some(String::from(since.to_str().unwrap()))
        }
    };



    let mut output = BufferedOutput::new();
    let reporter = HtmlReporter::new();

    match run_shtats(Path::new("."),
                     &mut output,
                     Box::new(reporter),
                     config) {
        Ok(_) => {
            println!("{}", output.to_string())
        }
        Err(_) => {
            println!("something went wrong");
        }
    }
}
