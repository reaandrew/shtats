use std::path::Path;
use std::process::exit;
use shtats::html::HtmlReporter;
use shtats::output::BufferedOutput;
use shtats::process::{get_number_of_commits, ProcessGitExecutor, Shtats};
use indicatif::ProgressBar;
use shtats::cli::Cli;
use shtats::config::Config;


fn main() {
    // TODO: Duplicate Commit Messages
    // TODO: Duplicate Commit Messages by user
    // TODO: Commits By Year if no time filter has been applied
    let cli = Cli::create();
    let config = Config::from(cli);
    match get_number_of_commits(){
        Ok(number_of_commits) => {
            let pb = ProgressBar::new(number_of_commits as u64);
            let mut reporter = HtmlReporter::new(BufferedOutput::new());
            let executor = Box::new(ProcessGitExecutor{});
            let process_callback = || pb.inc(1);
            let mut shtats = Shtats::create(&mut reporter, config, executor,&process_callback);

            match shtats.run(Path::new(".")) {
                Ok(_) => {
                    pb.finish_with_message("done");
                }
                Err(err) => {
                    eprintln!("error: {:?}", err);
                    exit(-1);
                }
            }
        }
        Err(err) => {
            eprintln!("error: {}", err);
        }
    }

}


