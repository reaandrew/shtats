use shtats::{BufferedOutput, HtmlReporter};

mod common;

#[test]
#[ignore]
fn test_cli_report() {
    common::setup_git_repo(|path| {
        let mut output = BufferedOutput::new();
        let reporter = HtmlReporter::new();
        shtats::run_shtats(path, &mut output, Box::new(reporter))?;
        println!("OUTPUT: {}", output.to_string());
        assert!(output.to_string().contains("<tr>
                    <td>Number of commits</td>
                    <td>3</td>
                </tr>"));

        Ok(())
    }).expect("fail");
}