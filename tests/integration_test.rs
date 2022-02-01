use forora::BufferedOutput;

mod common;

#[test]
fn test_cli_report() {
    common::setup_git_repo(|path|{
        let mut output = BufferedOutput::new();
        forora::run_forora(path, &mut output)?;
        assert_eq!("Count Count: 4", output.to_string());

        Ok(())
    }).expect("fail");
}