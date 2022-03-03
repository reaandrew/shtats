use shtats::html::HtmlReporter;
use shtats::output::BufferedOutput;
use shtats::process::Config;

mod common;

#[test]
#[ignore]
fn test_cli_report() {
    common::setup_git_repo(|path| {
        let mut output = BufferedOutput::new();
        let mut reporter = HtmlReporter::new(output);
        shtats::process::run_shtats(path, &mut reporter, Config{
            until: None,
            since: None,
            output: "".to_string()
        }, &|| {})?;

        assert!(output.to_string().contains("<tr>
                    <td>Number of commits</td>
                    <td>3</td>
                </tr>"));

        Ok(())
    }).expect("fail");
}