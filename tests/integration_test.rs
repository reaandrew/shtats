mod common;

#[test]
fn test_cli_report() {
    common::setup_git_repo(|path|{
        forora::run_forora(path)?;
        Ok(())
    }).expect("fail");
}