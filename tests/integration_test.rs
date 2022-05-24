use shtats::config::Config;
use shtats::html::HtmlReporter;
use shtats::output::BufferedOutput;
use shtats::process::{Shtats};
use shtats::Reporter;

mod common;

#[test]
#[ignore]
fn test_cli_report() {
    common::setup_git_repo(|path| {
        let output = BufferedOutput::new();
        let mut reporter = HtmlReporter::new(output);
        let config = Config{
            until: None,
            since: None,
            output: "".to_string()
        };
        let process_callback =  &|| {};
        let mut shtats = Shtats::create(&mut reporter, config, &process_callback);
        shtats.run(path)?;

        assert!(reporter.to_string().contains("<tr>
                    <td>Number of commits_collection</td>
                    <td>3</td>
                </tr>"));

        Ok(())
    }).expect("fail");
}